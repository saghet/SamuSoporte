use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodecError {
    #[error("Error de compresión LZ4: {0}")]
    Compression(String),

    #[error("Error de descompresión LZ4: {0}")]
    Decompression(String),

    #[error("Dimensiones de frame no válidas: {0}x{1}")]
    InvalidDimensions(u32, u32),

    #[error("Buffer de tamaño insuficiente. Esperado: {expected}, Provisto: {provided}")]
    BufferTooSmall { expected: usize, provided: usize },
}
