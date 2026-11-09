use crate::error::CryptoError;
use snow::{Builder, HandshakeState, StatelessTransportState};

pub static NOISE_PATTERN: &str = "Noise_NN_25519_ChaChaPoly_BLAKE2s";
pub static NOISE_XX_PATTERN: &str = "Noise_XX_25519_ChaChaPoly_BLAKE2s";

pub struct NoiseHandshake {
    state: HandshakeState,
}

impl NoiseHandshake {
    pub fn new_initiator() -> Result<Self, CryptoError> {
        let builder = Builder::new(NOISE_PATTERN.parse().unwrap());
        let state = builder
            .build_initiator()
            .map_err(|e| CryptoError::Noise(e.to_string()))?;

        Ok(Self { state })
    }

    pub fn new_responder() -> Result<Self, CryptoError> {
        let builder = Builder::new(NOISE_PATTERN.parse().unwrap());
        let state = builder
            .build_responder()
            .map_err(|e| CryptoError::Noise(e.to_string()))?;

        Ok(Self { state })
    }

    pub fn write_message(&mut self, payload: &[u8], message_out: &mut [u8]) -> Result<usize, CryptoError> {
        self.state
            .write_message(payload, message_out)
            .map_err(|e| CryptoError::Noise(e.to_string()))
    }

    pub fn read_message(&mut self, message_in: &[u8], payload_out: &mut [u8]) -> Result<usize, CryptoError> {
        self.state
            .read_message(message_in, payload_out)
            .map_err(|e| CryptoError::Noise(e.to_string()))
    }

    pub fn is_finished(&self) -> bool {
        self.state.is_handshake_finished()
    }

    pub fn get_handshake_hash(&self) -> Vec<u8> {
        self.state.get_handshake_hash().to_vec()
    }

    pub fn into_session(self) -> Result<NoiseSession, CryptoError> {
        let transport = self.state
            .into_stateless_transport_mode()
            .map_err(|e| CryptoError::Noise(e.to_string()))?;

        Ok(NoiseSession { transport, nonce_tx: 0, nonce_rx: 0 })
    }
}

pub struct NoiseSession {
    transport: StatelessTransportState,
    nonce_tx: u64,
    nonce_rx: u64,
}

impl NoiseSession {
    pub fn encrypt(&mut self, plaintext: &[u8], ciphertext_out: &mut [u8]) -> Result<usize, CryptoError> {
        let written = self.transport
            .write_message(self.nonce_tx, plaintext, ciphertext_out)
            .map_err(|e| CryptoError::Noise(e.to_string()))?;
        self.nonce_tx += 1;
        Ok(written)
    }

    pub fn decrypt(&mut self, ciphertext: &[u8], plaintext_out: &mut [u8]) -> Result<usize, CryptoError> {
        let read = self.transport
            .read_message(self.nonce_rx, ciphertext, plaintext_out)
            .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
        self.nonce_rx += 1;
        Ok(read)
    }
}
