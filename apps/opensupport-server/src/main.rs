use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

type Tx = mpsc::UnboundedSender<Message>;

#[derive(Default, Clone)]
pub struct ServerState {
    /// Mapeo de Device ID o Support Code a canal de envío
    clients: Arc<Mutex<HashMap<String, Tx>>>,
    /// Mapeo de Sesión ID a participantes (Iniciador, Respondedor)
    sessions: Arc<Mutex<HashMap<String, (String, Option<String>)>>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RelayMessage {
    Register {
        device_id: String,
        timestamp: Option<i64>,
    },
    SessionRequest {
        session_id: String,
        from_device: String,
        to_device: String,
        sas_code: Option<String>,
        timestamp: Option<i64>,
    },
    SessionResponse {
        session_id: String,
        from_device: String,
        to_device: String,
        accepted: bool,
        reason: Option<String>,
    },
    VideoFrame {
        session_id: String,
        from_device: String,
        to_device: String,
        data_base64: String,
    },
    InputEvent {
        session_id: String,
        from_device: String,
        to_device: String,
        event_type: i32,
        x: i32,
        y: i32,
        data: i32,
    },
    SessionTermination {
        session_id: String,
        from_device: String,
        to_device: String,
        reason: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = ServerState::default();
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/ws", get(ws_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("SamuSoporte Blind Relay Server escuchando en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_handler() -> &'static str {
    "OK - SamuSoporte Blind Relay Server 1.0.1 (Noise E2EE Ready)"
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<ServerState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: ServerState) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    let mut registered_device_id: Option<String> = None;

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            if let Ok(relay_msg) = serde_json::from_str::<RelayMessage>(&text) {
                match relay_msg {
                    RelayMessage::Register { device_id, .. } => {
                        let clean_id = device_id.replace(' ', "").to_lowercase();
                        info!("Cliente registrado: {} (normalizado: {})", device_id, clean_id);
                        registered_device_id = Some(clean_id.clone());
                        state.clients.lock().unwrap().insert(clean_id, tx.clone());
                    }
                    RelayMessage::SessionRequest {
                        ref session_id,
                        ref from_device,
                        ref to_device,
                        ..
                    } => {
                        let clean_to = to_device.replace(' ', "").to_lowercase();
                        let clean_from = from_device.replace(' ', "").to_lowercase();
                        registered_device_id = Some(clean_from.clone());
                        state.clients.lock().unwrap().insert(clean_from.clone(), tx.clone());

                        state.sessions.lock().unwrap().insert(
                            session_id.clone(),
                            (clean_from.clone(), Some(clean_to.clone())),
                        );

                        info!("Solicitud de sesión {} de {} hacia {}", session_id, clean_from, clean_to);

                        let target_tx = state.clients.lock().unwrap().get(&clean_to).cloned();
                        if let Some(target) = target_tx {
                            let _ = target.send(Message::Text(text.clone()));
                        } else {
                            warn!("Destino {} no conectado en el relay", clean_to);
                            let reject_msg = serde_json::json!({
                                "type": "SessionResponse",
                                "session_id": session_id,
                                "from_device": clean_to,
                                "to_device": clean_from,
                                "accepted": false,
                                "reason": "El equipo destino no está conectado o el código es incorrecto."
                            });
                            let _ = tx.send(Message::Text(reject_msg.to_string()));
                        }
                    }
                    RelayMessage::SessionResponse {
                        ref to_device,
                        ref from_device,
                        accepted,
                        ..
                    } => {
                        let clean_to = to_device.replace(' ', "").to_lowercase();
                        info!("Respuesta de sesión de {} hacia {}: accepted={}", from_device, clean_to, accepted);

                        let target_tx = state.clients.lock().unwrap().get(&clean_to).cloned();
                        if let Some(target) = target_tx {
                            let _ = target.send(Message::Text(text.clone()));
                        }
                    }
                    RelayMessage::VideoFrame { ref to_device, .. }
                    | RelayMessage::InputEvent { ref to_device, .. }
                    | RelayMessage::SessionTermination { ref to_device, .. } => {
                        let clean_to = to_device.replace(' ', "").to_lowercase();
                        let target_tx = state.clients.lock().unwrap().get(&clean_to).cloned();
                        if let Some(target) = target_tx {
                            let _ = target.send(Message::Text(text.clone()));
                        }
                    }
                }
            }
        }
    }

    if let Some(dev_id) = registered_device_id {
        info!("Cliente desconectado: {}", dev_id);
        state.clients.lock().unwrap().remove(&dev_id);
    }

    send_task.abort();
}
