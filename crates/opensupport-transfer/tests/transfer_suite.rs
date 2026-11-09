#[cfg(test)]
mod tests {
    use opensupport_transfer::{FileReceiver, FileSender, PathSecurityValidator};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_path_traversal_prevention() {
        let temp_dir = std::env::temp_dir().join("opensupport_test_transfer");
        let _ = tokio::fs::create_dir_all(&temp_dir).await;

        let validator = PathSecurityValidator::new(temp_dir.clone(), 10 * 1024 * 1024);

        // 1. Intento de Path Traversal
        let traversal_attempt = "../../Windows/System32/evil.dll";
        let res = validator.sanitize_and_resolve(traversal_attempt, 100);
        assert!(res.is_err(), "Path traversal con '..' debería ser bloqueado");

        // 2. Intento de extensión prohibida (.exe)
        let blocked_ext = "payload.exe";
        let res_ext = validator.sanitize_and_resolve(blocked_ext, 100);
        assert!(res_ext.is_err(), "Extensión .exe debería ser bloqueada");

        // 3. Archivo legítimo
        let valid = "informe_soporte.pdf";
        let res_valid = validator.sanitize_and_resolve(valid, 100);
        assert!(res_valid.is_ok());
        assert_eq!(res_valid.unwrap(), temp_dir.join("informe_soporte.pdf"));
    }

    #[tokio::test]
    async fn test_file_transfer_integrity_and_sha256() {
        let temp_dir = std::env::temp_dir().join("opensupport_test_e2e_transfer");
        let _ = tokio::fs::create_dir_all(&temp_dir).await;

        let source_file = temp_dir.join("source.txt");
        let payload = b"Transferencia segura de archivos para OpenSupport - Hash Check SHA256";
        tokio::fs::write(&source_file, payload).await.unwrap();

        let sender = FileSender::new(&source_file, 16).await.unwrap();
        assert_eq!(sender.total_size, payload.len() as u64);

        let dest_dir = temp_dir.join("destination");
        let _ = tokio::fs::create_dir_all(&dest_dir).await;

        let validator = PathSecurityValidator::new(dest_dir.clone(), 10 * 1024 * 1024);
        let mut receiver = FileReceiver::new(validator);

        let transfer_id = Uuid::new_v4();
        let _ = receiver
            .start_transfer(transfer_id, "received.txt", sender.total_size, sender.sha256)
            .await
            .unwrap();

        // Enviar chunks secuenciales
        let mut offset = 0;
        loop {
            let chunk = sender.read_chunk(offset, 16).await.unwrap();
            if chunk.is_empty() {
                break;
            }
            receiver.process_chunk(transfer_id, offset, &chunk).await.unwrap();
            offset += chunk.len() as u64;
        }

        let saved_path = receiver.finish_transfer(transfer_id).await.unwrap();
        let saved_content = tokio::fs::read(&saved_path).await.unwrap();
        assert_eq!(saved_content, payload);
    }
}
