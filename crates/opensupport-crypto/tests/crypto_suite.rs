#[cfg(test)]
mod tests {
    use opensupport_crypto::{DeviceIdentity, NoiseHandshake, ShortAuthString};
    use opensupport_crypto::transport::SecureChannel;

    #[test]
    fn test_device_identity_generation_and_signatures() {
        let identity = DeviceIdentity::generate();
        let msg = b"OpenSupport Security Test Message 2026";
        
        let signature = identity.sign(msg);
        let pubkey = identity.verifying_key_bytes();

        // Verificación válida
        let res = DeviceIdentity::verify_signature(&pubkey, msg, &signature);
        assert!(res.is_ok(), "Firma criptográfica válida falló verificación");

        // Verificación con mensaje alterado (Anti-Tampering)
        let tampered = b"OpenSupport Security Test Message 2026 - Altered";
        let res_tampered = DeviceIdentity::verify_signature(&pubkey, tampered, &signature);
        assert!(res_tampered.is_err(), "Mensaje manipulado pasó verificación erróneamente");
    }

    #[test]
    fn test_noise_handshake_and_secure_channel_e2ee() {
        let mut initiator = NoiseHandshake::new_initiator().expect("Initiator init failed");
        let mut responder = NoiseHandshake::new_responder().expect("Responder init failed");

        let mut buf1 = [0u8; 1024];
        let mut buf2 = [0u8; 1024];
        let mut payload_buf = [0u8; 1024];

        // Paso 1: Iniciador -> Respondedor (e)
        let len1 = initiator.write_message(b"", &mut buf1).expect("Init write step 1");
        responder.read_message(&buf1[..len1], &mut payload_buf).expect("Resp read step 1");

        // Paso 2: Respondedor -> Iniciador (e, ee)
        let len2 = responder.write_message(b"", &mut buf2).expect("Resp write step 2");
        initiator.read_message(&buf2[..len2], &mut payload_buf).expect("Init read step 2");

        assert!(initiator.is_finished());
        assert!(responder.is_finished());

        // Verificar Short Authentication String (SAS)
        let hash_init = initiator.get_handshake_hash();
        let hash_resp = responder.get_handshake_hash();
        assert_eq!(hash_init, hash_resp, "Handshake hash no coincide entre extremos");

        let sas_code_init = ShortAuthString::generate_numeric(&hash_init);
        let sas_code_resp = ShortAuthString::generate_numeric(&hash_resp);
        assert_eq!(sas_code_init, sas_code_resp);
        assert_eq!(sas_code_init.len(), 6);

        let sas_words_init = ShortAuthString::generate_words(&hash_init);
        let sas_words_resp = ShortAuthString::generate_words(&hash_resp);
        assert_eq!(sas_words_init, sas_words_resp);

        // Convertir en canales de transporte cifrado
        let session_init = initiator.into_session().expect("Initiator session");
        let session_resp = responder.into_session().expect("Responder session");

        let mut channel_client = SecureChannel::new(session_init);
        let mut channel_server = SecureChannel::new(session_resp);

        // Prueba de transmisión de carga útil cifrada
        let secret_command = b"EXECUTE_REMOTE_ASSISTANCE_SESSION_TOKEN_XYZ";
        let encrypted = channel_client.encrypt_packet(secret_command).expect("Encrypt failed");

        // Comprobar que el texto cifrado es diferente del texto claro
        assert_ne!(&encrypted[..], secret_command);

        // Descifrado por el otro extremo
        let decrypted = channel_server.decrypt_packet(&encrypted).expect("Decrypt failed");
        assert_eq!(&decrypted[..], secret_command);
    }
}
