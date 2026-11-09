use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[derive(Error, Debug)]
pub enum AuditError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Verificación de cadena criptográfica fallida en registro #{0}")]
    ChainIntegrityViolation(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditEventType {
    SessionStarted,
    SessionTerminated,
    PermissionsGranted,
    FileTransferStarted,
    FileTransferCompleted,
    AdministrativeAction,
    FailedAuthAttempt,
    SecurityAlert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub sequence: u64,
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub technician_id: String,
    pub target_device_id: String,
    pub client_ip: String,
    pub details: serde_json::Value,
    pub previous_entry_hash: [u8; 32],
    pub entry_hash: [u8; 32],
}

impl AuditEntry {
    pub fn calculate_hash(
        sequence: u64,
        timestamp: &DateTime<Utc>,
        event_type: &AuditEventType,
        technician_id: &str,
        target_device_id: &str,
        client_ip: &str,
        details: &serde_json::Value,
        prev_hash: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(sequence.to_be_bytes());
        hasher.update(timestamp.to_rfc3339().as_bytes());
        hasher.update(format!("{:?}", event_type).as_bytes());
        hasher.update(technician_id.as_bytes());
        hasher.update(target_device_id.as_bytes());
        hasher.update(client_ip.as_bytes());
        hasher.update(details.to_string().as_bytes());
        hasher.update(prev_hash);
        hasher.finalize().into()
    }
}

pub struct AuditLogger {
    log_path: PathBuf,
    current_sequence: u64,
    last_hash: [u8; 32],
}

impl AuditLogger {
    pub async fn init<P: AsRef<Path>>(path: P) -> Result<Self, AuditError> {
        let p = path.as_ref().to_path_buf();
        if let Some(parent) = p.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut logger = Self {
            log_path: p,
            current_sequence: 0,
            last_hash: [0u8; 32],
        };

        // Si ya existe el archivo, verificar integridad de la cadena
        if logger.log_path.exists() {
            logger.verify_and_load_chain().await?;
        }

        Ok(logger)
    }

    pub async fn log_event(
        &mut self,
        event_type: AuditEventType,
        technician_id: &str,
        target_device_id: &str,
        client_ip: &str,
        details: serde_json::Value,
    ) -> Result<AuditEntry, AuditError> {
        self.current_sequence += 1;
        let timestamp = Utc::now();
        let entry_hash = AuditEntry::calculate_hash(
            self.current_sequence,
            &timestamp,
            &event_type,
            technician_id,
            target_device_id,
            client_ip,
            &details,
            &self.last_hash,
        );

        let entry = AuditEntry {
            sequence: self.current_sequence,
            timestamp,
            event_type,
            technician_id: technician_id.to_string(),
            target_device_id: target_device_id.to_string(),
            client_ip: client_ip.to_string(),
            details,
            previous_entry_hash: self.last_hash,
            entry_hash,
        };

        self.last_hash = entry_hash;

        let mut line = serde_json::to_string(&entry)?;
        line.push('\n');

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
            .await?;
        file.write_all(line.as_bytes()).await?;
        file.flush().await?;

        Ok(entry)
    }

    pub async fn verify_and_load_chain(&mut self) -> Result<u64, AuditError> {
        let content = tokio::fs::read_to_string(&self.log_path).await?;
        let mut expected_prev = [0u8; 32];
        let mut count = 0;

        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let entry: AuditEntry = serde_json::from_str(line)?;
            if entry.previous_entry_hash != expected_prev {
                return Err(AuditError::ChainIntegrityViolation(entry.sequence));
            }

            let recalculated = AuditEntry::calculate_hash(
                entry.sequence,
                &entry.timestamp,
                &entry.event_type,
                &entry.technician_id,
                &entry.target_device_id,
                &entry.client_ip,
                &entry.details,
                &entry.previous_entry_hash,
            );

            if recalculated != entry.entry_hash {
                return Err(AuditError::ChainIntegrityViolation(entry.sequence));
            }

            expected_prev = entry.entry_hash;
            count = entry.sequence;
        }

        self.current_sequence = count;
        self.last_hash = expected_prev;
        Ok(count)
    }
}
