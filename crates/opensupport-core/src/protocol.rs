use serde::{Deserialize, Serialize};
use crate::types::{DeviceId, SessionId, SessionPermissions};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalMessage {
    /// Solicitud de inicio de sesión con clave pública efímera
    SessionRequest {
        session_id: SessionId,
        from_device: DeviceId,
        to_device: DeviceId,
        ephemeral_public_key: Vec<u8>,
        requested_permissions: SessionPermissions,
    },
    /// Respuesta a la solicitud
    SessionResponse {
        session_id: SessionId,
        accepted: bool,
        ephemeral_public_key: Vec<u8>,
        granted_permissions: SessionPermissions,
        reason: Option<String>,
    },
    /// Candidatos ICE / STUN / TURN para negociación de conexión P2P
    IceCandidate {
        session_id: SessionId,
        candidate: String,
    },
    /// Paquete Relay (datos cifrados E2EE en tránsito sin inspección por el servidor)
    RelayPayload {
        session_id: SessionId,
        payload: Vec<u8>,
    },
    /// Cierre inmediato de sesión (parada de emergencia o finalización)
    SessionTermination {
        session_id: SessionId,
        reason: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataChannelMessage {
    /// Ping / Keepalive
    Ping(u64),
    Pong(u64),

    /// Cuadro de vídeo o frame diferencial
    VideoFrame {
        timestamp: u64,
        keyframe: bool,
        width: u32,
        height: u32,
        data: Vec<u8>,
    },

    /// Eventos de entrada (teclado / mouse)
    Input(InputEvent),

    /// Transferencia de archivos
    FileTransfer(FileTransferMessage),

    /// Clipboard
    Clipboard {
        text: String,
    },

    /// Diagnóstico / Herramientas
    Diagnostics(DiagnosticsMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputEvent {
    MouseMove { x: i32, y: i32 },
    MouseDown { button: MouseButton },
    MouseUp { button: MouseButton },
    MouseWheel { delta_x: i32, delta_y: i32 },
    KeyDown { key_code: u32, scan_code: u32 },
    KeyUp { key_code: u32, scan_code: u32 },
    SpecialCombo(SpecialKeyCombo),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Extra(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialKeyCombo {
    CtrlAltDel,
    WinL,
    AltTab,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileTransferMessage {
    RequestUpload {
        transfer_id: uuid::Uuid,
        filename: String,
        total_size: u64,
        sha256: [u8; 32],
    },
    UploadAccept {
        transfer_id: uuid::Uuid,
        resume_offset: u64,
    },
    UploadReject {
        transfer_id: uuid::Uuid,
        reason: String,
    },
    Chunk {
        transfer_id: uuid::Uuid,
        offset: u64,
        data: Vec<u8>,
    },
    Complete {
        transfer_id: uuid::Uuid,
        final_sha256: [u8; 32],
    },
    Cancel {
        transfer_id: uuid::Uuid,
        reason: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticsMessage {
    RequestSystemInfo,
    SystemInfoResponse {
        os: String,
        hostname: String,
        cpu_usage_pct: f32,
        mem_total_bytes: u64,
        mem_used_bytes: u64,
    },
    RequestProcessList,
    ProcessListResponse(Vec<ProcessSummary>),
    KillProcessRequest { pid: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSummary {
    pub pid: u32,
    pub name: String,
    pub cpu_pct: f32,
    pub memory_bytes: u64,
}
