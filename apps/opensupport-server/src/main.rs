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
use opensupport_core::protocol::SignalMessage;
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
    /// Mapeo de Device ID en formato Hex a canal de envío
    clients: Arc<Mutex<HashMap<String, Tx>>>,
    /// Mapeo de Sesión ID (String) a participantes (Iniciador Hex, Respondedor Hex)
    sessions: Arc<Mutex<HashMap<String, (String, Option<String>)>>>,
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
    info!("OpenSupport Rendezvous & Blind Relay Server escuchando en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_handler() -> &'static str {
    "OK - OpenSupport Server 1.0 (Zero-Knowledge Relay)"
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

    let mut current_device_id: Option<String> = None;

    // Tarea encargada de enviar paquetes al WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Procesar mensajes entrantes del cliente
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            match serde_json::from_str::<SignalMessage>(&text) {
                Ok(signal) => match signal {
                    SignalMessage::SessionRequest {
                        session_id,
                        from_device,
                        to_device,
                        ephemeral_public_key,
                        requested_permissions,
                    } => {
                        let from_hex = from_device.to_hex();
                        let to_hex = to_device.to_hex();
                        current_device_id = Some(from_hex.clone());

                        // Registrar cliente
                        state
                            .clients
                            .lock()
                            .unwrap()
                            .insert(from_hex.clone(), tx.clone());

                        // Registrar sesión
                        state
                            .sessions
                            .lock()
                            .unwrap()
                            .insert(session_id.to_string(), (from_hex.clone(), Some(to_hex.clone())));

                        info!(
                            "Solicitud de sesión {} de {} para {}",
                            session_id, from_hex, to_hex
                        );

                        // Enrutar al target si está conectado
                        let target_tx = state.clients.lock().unwrap().get(&to_hex).cloned();
                        if let Some(target) = target_tx {
                            let forward_msg = SignalMessage::SessionRequest {
                                session_id,
                                from_device,
                                to_device,
                                ephemeral_public_key,
                                requested_permissions,
                            };
                            let _ = target.send(Message::Text(serde_json::to_string(&forward_msg).unwrap()));
                        } else {
                            warn!("Target {} no conectado actualmente", to_hex);
                        }
                    }
                    SignalMessage::SessionResponse {
                        session_id,
                        accepted,
                        ephemeral_public_key,
                        granted_permissions,
                        reason,
                    } => {
                        let session_info = state
                            .sessions
                            .lock()
                            .unwrap()
                            .get(&session_id.to_string())
                            .cloned();

                        if let Some((initiator_hex, _)) = session_info {
                            let initiator_tx = state.clients.lock().unwrap().get(&initiator_hex).cloned();
                            if let Some(target) = initiator_tx {
                                let forward_msg = SignalMessage::SessionResponse {
                                    session_id,
                                    accepted,
                                    ephemeral_public_key,
                                    granted_permissions,
                                    reason,
                                };
                                let _ = target.send(Message::Text(serde_json::to_string(&forward_msg).unwrap()));
                            }
                        }
                    }
                    SignalMessage::RelayPayload {
                        session_id,
                        payload,
                    } => {
                        // Enrutamiento Ciego (Zero-Knowledge Relay): el servidor no descifra la carga útil
                        let session_info = state
                            .sessions
                            .lock()
                            .unwrap()
                            .get(&session_id.to_string())
                            .cloned();

                        if let Some((initiator_hex, Some(responder_hex))) = session_info {
                            let sender_id = current_device_id.as_deref().unwrap_or("");
                            let dest_hex = if sender_id == initiator_hex {
                                responder_hex
                            } else {
                                initiator_hex
                            };

                            let dest_tx = state.clients.lock().unwrap().get(&dest_hex).cloned();
                            if let Some(target) = dest_tx {
                                let forward = SignalMessage::RelayPayload {
                                    session_id,
                                    payload,
                                };
                                let _ = target.send(Message::Text(serde_json::to_string(&forward).unwrap()));
                            }
                        }
                    }
                    SignalMessage::IceCandidate {
                        session_id,
                        candidate,
                    } => {
                        let session_info = state
                            .sessions
                            .lock()
                            .unwrap()
                            .get(&session_id.to_string())
                            .cloned();

                        if let Some((initiator_hex, Some(responder_hex))) = session_info {
                            let sender_id = current_device_id.as_deref().unwrap_or("");
                            let dest_hex = if sender_id == initiator_hex {
                                responder_hex
                            } else {
                                initiator_hex
                            };

                            let dest_tx = state.clients.lock().unwrap().get(&dest_hex).cloned();
                            if let Some(target) = dest_tx {
                                let forward = SignalMessage::IceCandidate {
                                    session_id,
                                    candidate,
                                };
                                let _ = target.send(Message::Text(serde_json::to_string(&forward).unwrap()));
                            }
                        }
                    }
                    SignalMessage::SessionTermination { session_id, reason } => {
                        info!("Sesión {} terminada: {}", session_id, reason);
                        state.sessions.lock().unwrap().remove(&session_id.to_string());
                    }
                },
                Err(e) => {
                    error!("Error deserializando SignalMessage: {:?}", e);
                }
            }
        }
    }

    send_task.abort();
    if let Some(dev_id) = current_device_id {
        state.clients.lock().unwrap().remove(&dev_id);
    }
}
