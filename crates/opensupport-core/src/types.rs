use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub [u8; 32]);

impl DeviceId {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn to_hex(&self) -> String {
        self.0.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

impl std::fmt::Display for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionType {
    Attended,
    Unattended,
    Enterprise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPermissions {
    pub view_screen: bool,
    pub input_control: bool,
    pub file_transfer: bool,
    pub clipboard_sync: bool,
    pub system_diagnostics: bool,
    pub restart_reboot: bool,
    pub lock_session: bool,
}

impl Default for SessionPermissions {
    fn default() -> Self {
        Self {
            view_screen: true,
            input_control: false,
            file_transfer: false,
            clipboard_sync: false,
            system_diagnostics: false,
            restart_reboot: false,
            lock_session: false,
        }
    }
}

impl SessionPermissions {
    pub fn full_control() -> Self {
        Self {
            view_screen: true,
            input_control: true,
            file_transfer: true,
            clipboard_sync: true,
            system_diagnostics: true,
            restart_reboot: true,
            lock_session: true,
        }
    }

    pub fn view_only() -> Self {
        Self {
            view_screen: true,
            input_control: false,
            file_transfer: false,
            clipboard_sync: false,
            system_diagnostics: false,
            restart_reboot: false,
            lock_session: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub session_id: SessionId,
    pub session_type: SessionType,
    pub technician_id: String,
    pub target_device_id: DeviceId,
    pub created_at: DateTime<Utc>,
    pub permissions: SessionPermissions,
    pub short_auth_string: String,
}
