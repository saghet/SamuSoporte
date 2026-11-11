# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0-alpha] - 2026-11-10

### Added
- **Native Desktop Client (.NET 10 / WPF-UI):**
  - Fully native Windows 11 Fluent interface with Mica backdrop and system titlebar integration.
  - Interactive multi-view navigation system (`Panel Principal`, `Soporte Remoto`, `Mis Dispositivos`, `Consola Técnico`, `Seguridad & Zero Trust`, `Auditoría Inmutable`, `Configuración`).
  - Dynamic 6-digit support code generator with 15-minute expiration timer and clipboard copy.
  - Immediate Emergency Stop button for attended session termination.
  - Custom application branding and icon integration (`AppIcon.ico`).
- **Cryptographic & Zero Trust Core (`crates/opensupport-crypto`):**
  - End-to-End Encryption utilizing Noise Protocol Framework (`Noise_XX_25519_ChaChaPoly_BLAKE2s`).
  - Device identity generation with Ed25519 signing and verification.
  - Short Authentication String (SAS) numeric code derivation for Anti-MitM mutual authentication.
  - In-memory key zeroization on drop with `zeroize` crate.
- **Diagnostics & Forensic Auditing (`crates/opensupport-diagnostics` & `crates/opensupport-audit`):**
  - Hardware telemetry collection (CPU usage, RAM allocation, NVMe storage metrics).
  - SHA-256 hash-chained immutable audit log for all security events.
- **File Transfer & Input Engines (`crates/opensupport-transfer` & `crates/opensupport-input`):**
  - Chunked file transfer protocol with path traversal prevention and SHA-256 validation.
  - Windows `SendInput` mouse and keyboard translation.
- **Relay Server & Windows Service (`apps/opensupport-server` & `apps/opensupport-service`):**
  - Zero-Knowledge WebSocket relay router with rate limiting and authentication hooks.
  - Windows background service architecture with IPC Named Pipe foundation.
- **DevSecOps & Community:**
  - Automated CI/CD workflows for building and testing .NET and Rust components.
  - Open source governance files: AGPL-3.0 License, Security Policy, Code of Conduct, and Contributing Guidelines.
