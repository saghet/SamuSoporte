use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Path Traversal / Inseguridad detectada en la ruta: {0}")]
    SecurityViolation(String),

    #[error("Extensión de archivo prohibida por política: {0}")]
    BlockedExtension(String),

    #[error("Tamaño excede el límite permitido ({0} bytes)")]
    FileTooLarge(u64),

    #[error("Verificación de integridad SHA-256 fallida. Esperado: {expected}, Calculado: {calculated}")]
    IntegrityCheckFailed { expected: String, calculated: String },

    #[error("Transferencia no encontrada o cancelada: {0}")]
    TransferNotFound(uuid::Uuid),

    #[error("Offset de chunk inválido. Esperado: {expected}, Recibido: {received}")]
    InvalidOffset { expected: u64, received: u64 },
}
