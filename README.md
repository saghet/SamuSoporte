# OpenSupport: Plataforma Open Source de Soporte Remoto Profesional

> **Segura &bull; Cifrado E2EE con Noise Protocol &bull; Zero-Knowledge Relay &bull; Zero Trust Architecture**

OpenSupport es una solución de asistencia y gestión remota de equipos diseñada para competir directamente con plataformas comerciales como AnyDesk, TeamViewer o RustDesk, priorizando la privacidad, la seguridad de memoria (implementada en Rust) y la soberanía total de la infraestructura.

---

## 🚀 Arquitectura y Componentes Desarrollados

El monorepo está estructurado en crates independientes y testeados:

- **`crates/opensupport-core`**: Modelos de sesión, tipos de dispositivos, mensajes de señalización y canales de datos.
- **`crates/opensupport-crypto`**: Cifrado de Extremo a Extremo (**E2EE**) mediante el **Noise Protocol Framework** (`Noise_NN_25519_ChaChaPoly_BLAKE2s`), firmas digitales estrictas Ed25519 y códigos de autenticación visual/fonética (**SAS anti-MitM**).
- **`crates/opensupport-input`**: Inyección nativa en Windows con `SendInput`, compatibilidad con escritorios virtuales y multimonitor, y control seguro de permisos UIPI.
- **`crates/opensupport-transfer`**: Motor asíncrono de transferencia de archivos bidireccional con mitigación activa de **Path Traversal**, bloqueo de extensiones de riesgo, verificación de integridad SHA-256 por bloques y reanudación de descargas interrumpidas.
- **`crates/opensupport-diagnostics`**: Caja de herramientas técnicas: lectura en tiempo real de CPU, memoria RAM, discos, interfaces de red y terminación segura de procesos por PID.
- **`crates/opensupport-audit`**: Bitácora de auditoría inmutable con cadena de hashes encadenados criptográficamente (Tamper-evident Audit Ledger).
- **`apps/opensupport-server`**: Servidor central Rendezvous y **Blind Relay (Zero-Knowledge)** construido con Axum y WebSockets de alto rendimiento.
- **`apps/opensupport-desktop`**: Interfaz gráfica moderna (Host Asistido, Consola de Técnico, Auditoría y Banner de Sesión Activa).

---

## 🧪 Verificación y Pruebas Automatizadas

Para compilar y ejecutar la suite completa de pruebas unitarias y de integración:

```bash
cargo test --workspace
```

Resultados obtenidos en laboratorio:
```text
test tests::test_noise_handshake_and_secure_channel_e2ee ... ok
test tests::test_device_identity_generation_and_signatures ... ok
test tests::test_diagnostics_collector_processes ... ok
test tests::test_diagnostics_collector_system_overview ... ok
test tests::test_input_injector_instantiation_and_structure ... ok
test tests::test_path_traversal_prevention ... ok
test tests::test_file_transfer_integrity_and_sha256 ... ok
test tests::test_audit_hash_chain_integrity ... ok
```

---

## 🐳 Despliegue de Servidor Autohospedado

Para desplegar tu propio servidor Relay y de Señalización con Docker Compose:

```bash
cd deploy/docker
docker compose up -d --build
```

El servidor estará escuchando en el puerto `8080` con endpoint de diagnóstico en `http://localhost:8080/health`.

---

## 📜 Licencia

Distribuido bajo la licencia **GNU Affero General Public License v3.0 (AGPL-3.0)**.
