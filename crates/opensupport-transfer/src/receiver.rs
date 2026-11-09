use crate::error::TransferError;
use crate::validator::PathSecurityValidator;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub struct IncomingSession {
    pub target_path: PathBuf,
    pub file: File,
    pub total_size: u64,
    pub written_bytes: u64,
    pub expected_sha256: [u8; 32],
    pub hasher: Sha256,
}

pub struct FileReceiver {
    validator: PathSecurityValidator,
    active_transfers: HashMap<Uuid, IncomingSession>,
}

impl FileReceiver {
    pub fn new(validator: PathSecurityValidator) -> Self {
        Self {
            validator,
            active_transfers: HashMap::new(),
        }
    }

    pub async fn start_transfer(
        &mut self,
        transfer_id: Uuid,
        filename: &str,
        total_size: u64,
        expected_sha256: [u8; 32],
    ) -> Result<u64, TransferError> {
        let target_path = self.validator.sanitize_and_resolve(filename, total_size)?;

        let (file, resume_offset) = if target_path.exists() {
            let metadata = tokio::fs::metadata(&target_path).await?;
            let existing_len = metadata.len();
            if existing_len < total_size {
                let f = OpenOptions::new().append(true).open(&target_path).await?;
                (f, existing_len)
            } else {
                let f = File::create(&target_path).await?;
                (f, 0)
            }
        } else {
            let f = File::create(&target_path).await?;
            (f, 0)
        };

        let session = IncomingSession {
            target_path,
            file,
            total_size,
            written_bytes: resume_offset,
            expected_sha256,
            hasher: Sha256::new(),
        };

        self.active_transfers.insert(transfer_id, session);
        Ok(resume_offset)
    }

    pub async fn process_chunk(
        &mut self,
        transfer_id: Uuid,
        offset: u64,
        data: &[u8],
    ) -> Result<(), TransferError> {
        let session = self
            .active_transfers
            .get_mut(&transfer_id)
            .ok_or(TransferError::TransferNotFound(transfer_id))?;

        if offset != session.written_bytes {
            return Err(TransferError::InvalidOffset {
                expected: session.written_bytes,
                received: offset,
            });
        }

        session.file.write_all(data).await?;
        session.written_bytes += data.len() as u64;
        session.hasher.update(data);
        Ok(())
    }

    pub async fn finish_transfer(&mut self, transfer_id: Uuid) -> Result<PathBuf, TransferError> {
        let mut session = self
            .active_transfers
            .remove(&transfer_id)
            .ok_or(TransferError::TransferNotFound(transfer_id))?;

        session.file.flush().await?;

        let calculated_hash: [u8; 32] = session.hasher.finalize().into();
        if calculated_hash != session.expected_sha256 {
            // Eliminar archivo corrupto o manipulado por seguridad
            let _ = tokio::fs::remove_file(&session.target_path).await;
            return Err(TransferError::IntegrityCheckFailed {
                expected: session.expected_sha256.iter().map(|b| format!("{:02x}", b)).collect(),
                calculated: calculated_hash.iter().map(|b| format!("{:02x}", b)).collect(),
            });
        }

        Ok(session.target_path)
    }
}
