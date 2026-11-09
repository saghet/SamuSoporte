use crate::error::CryptoError;
use crate::noise::NoiseSession;

pub struct SecureChannel {
    session: NoiseSession,
}

impl SecureChannel {
    pub fn new(session: NoiseSession) -> Self {
        Self { session }
    }

    pub fn encrypt_packet(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut out = vec![0u8; plaintext.len() + 16]; // ChaChaPoly tag es 16 bytes
        let written = self.session.encrypt(plaintext, &mut out)?;
        out.truncate(written);
        Ok(out)
    }

    pub fn decrypt_packet(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if ciphertext.len() < 16 {
            return Err(CryptoError::DecryptionFailed("Ciphertext too short".into()));
        }
        let mut out = vec![0u8; ciphertext.len()];
        let read = self.session.decrypt(ciphertext, &mut out)?;
        out.truncate(read);
        Ok(out)
    }
}
