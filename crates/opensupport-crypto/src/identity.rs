use crate::error::CryptoError;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use opensupport_core::types::DeviceId;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

pub struct DeviceIdentity {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    device_id: DeviceId,
}

impl DeviceIdentity {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        
        let mut hasher = Sha256::new();
        hasher.update(verifying_key.as_bytes());
        let hash: [u8; 32] = hasher.finalize().into();
        let device_id = DeviceId::from_bytes(hash);

        Self {
            signing_key,
            verifying_key,
            device_id,
        }
    }

    pub fn device_id(&self) -> DeviceId {
        self.device_id
    }

    pub fn verifying_key_bytes(&self) -> [u8; 32] {
        *self.verifying_key.as_bytes()
    }

    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        let signature: Signature = self.signing_key.sign(message);
        signature.to_bytes()
    }

    pub fn verify_signature(
        public_key_bytes: &[u8; 32],
        message: &[u8],
        signature_bytes: &[u8; 64],
    ) -> Result<(), CryptoError> {
        let verifying_key = VerifyingKey::from_bytes(public_key_bytes)
            .map_err(|e| CryptoError::KeyFormatError(e.to_string()))?;
        let signature = Signature::from_bytes(signature_bytes);

        verifying_key
            .verify_strict(message, &signature)
            .map_err(|e| CryptoError::IdentityVerificationFailed(e.to_string()))
    }
}
