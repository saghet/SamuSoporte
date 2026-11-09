use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Noise Protocol error: {0}")]
    Noise(String),

    #[error("Key exchange error: {0}")]
    KeyExchange(String),

    #[error("Decryption / MAC verification failed: {0}")]
    DecryptionFailed(String),

    #[error("Identity verification failed: {0}")]
    IdentityVerificationFailed(String),

    #[error("Serialization / Key format error: {0}")]
    KeyFormatError(String),
}
