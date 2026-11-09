#[cfg(test)]
mod tests {
    use opensupport_audit::{AuditEventType, AuditLogger};
    use serde_json::json;

    #[tokio::test]
    async fn test_audit_hash_chain_integrity() {
        let temp_dir = std::env::temp_dir().join("opensupport_test_audit");
        let _ = tokio::fs::create_dir_all(&temp_dir).await;
        let log_file = temp_dir.join("audit_ledger.jsonl");
        let _ = tokio::fs::remove_file(&log_file).await;

        let mut logger = AuditLogger::init(&log_file).await.unwrap();

        // Registrar 3 eventos reales
        logger
            .log_event(
                AuditEventType::SessionStarted,
                "tech_alice@corp.com",
                "device_node_8841",
                "192.168.1.50",
                json!({ "mode": "Attended", "permissions": "FullControl" }),
            )
            .await
            .unwrap();

        logger
            .log_event(
                AuditEventType::FileTransferStarted,
                "tech_alice@corp.com",
                "device_node_8841",
                "192.168.1.50",
                json!({ "filename": "patch_v2.bin", "size": 1048576 }),
            )
            .await
            .unwrap();

        logger
            .log_event(
                AuditEventType::SessionTerminated,
                "tech_alice@corp.com",
                "device_node_8841",
                "192.168.1.50",
                json!({ "duration_secs": 420 }),
            )
            .await
            .unwrap();

        // 1. Verificación exitosa de la cadena
        let mut verifier = AuditLogger::init(&log_file).await.unwrap();
        let verified_count = verifier.verify_and_load_chain().await.unwrap();
        assert_eq!(verified_count, 3);

        // 2. Simular intento de manipulación no autorizada (Tampering)
        let log_content = tokio::fs::read_to_string(&log_file).await.unwrap();
        let tampered_content = log_content.replace("patch_v2.bin", "malicious_rootkit.bin");
        tokio::fs::write(&log_file, tampered_content).await.unwrap();

        // 3. Comprobar que el verificador detecta la alteración inmediatamente
        let tampered_verifier = AuditLogger::init(&log_file).await;
        assert!(
            tampered_verifier.is_err(),
            "La alteración de la bitácora debió ser detectada por la cadena de hashes"
        );
    }
}
