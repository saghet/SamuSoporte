use crate::error::TransferError;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub struct FileSender {
    pub file_path: PathBuf,
    pub filename: String,
    pub total_size: u64,
    pub sha256: [u8; 32],
    pub chunk_size: usize,
}

impl FileSender {
    pub async fn new<P: AsRef<Path>>(path: P, chunk_size: usize) -> Result<Self, TransferError> {
        let p = path.as_ref().to_path_buf();
        let metadata = tokio::fs::metadata(&p).await?;
        let total_size = metadata.len();

        let filename = p
            .file_name()
            .ok_or_else(|| TransferError::SecurityViolation("Nombre de archivo inválido".into()))?
            .to_string_lossy()
            .to_string();

        // Calcular SHA-256 de forma asíncrona
        let mut file = File::open(&p).await?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 64 * 1024];

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let sha256 = hasher.finalize().into();

        Ok(Self {
            file_path: p,
            filename,
            total_size,
            sha256,
            chunk_size,
        })
    }

    pub async fn read_chunk(&self, offset: u64, size: usize) -> Result<Vec<u8>, TransferError> {
        use std::io::SeekFrom;
        use tokio::io::AsyncSeekExt;

        let mut file = File::open(&self.file_path).await?;
        file.seek(SeekFrom::Start(offset)).await?;

        let mut buf = vec![0u8; size];
        let bytes_read = file.read(&mut buf).await?;
        buf.truncate(bytes_read);
        Ok(buf)
    }
}
