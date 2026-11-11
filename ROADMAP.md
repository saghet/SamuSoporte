# SamuSoporte Roadmap

This document outlines the engineering and security milestones for **SamuSoporte**. Development follows strict Zero Trust principles, native Windows integration, and high-performance remote desktop protocols.

---

## 🟢 Implemented & Verified (v0.1.0-alpha)

### Core Architecture & Cryptography
- [x] **End-to-End Encryption (E2EE):** Noise Protocol Framework (`Noise_XX_25519_ChaChaPoly_BLAKE2s`) in Rust (`opensupport-crypto`).
- [x] **Anti-MitM Mutual Authentication:** Short Authentication String (SAS) numeric verification.
- [x] **Memory Protection:** Key material scrubbed upon drop using `Zeroize` (`opensupport-crypto`).
- [x] **Immutable Forensic Audit Trail:** Hash-chained SHA-256 tamper-evident event logging (`opensupport-audit`).

### Native Desktop Experience (.NET 10 / WPF-UI)
- [x] **Fluent UI Windows 11 Native Client:** Hardware-accelerated Mica/Acrylic windowing. No Electron, no web views.
- [x] **Interactive Sidebar Navigation:**
  - **Panel Principal:** 6-digit dynamic support code generator, direct connection, emergency killswitch.
  - **Soporte Remoto:** Multi-monitor switching, clipboard sync toggle, file transfer initialization, SAS Ctrl+Alt+Del injection.
  - **Mis Dispositivos:** Device inventory with real-time status and Wake-on-LAN (WoL) triggers.
  - **Consola del Técnico:** Real-time CPU, RAM, and NVMe telemetry, process explorer shortcut.
  - **Seguridad & Zero Trust:** Real-time cryptographic status, active public keys, and Zero Trust access policies.
  - **Auditoría Inmutable:** Live forensic event stream with SHA-256 verification and SIEM export (JSON-L).
  - **Configuración:** Self-hosted relay endpoint configuration and GPU hardware acceleration options.

### Communication & Services
- [x] **Self-Hosted Relay Server:** Zero-Knowledge WebSocket relay router (`opensupport-server`).
- [x] **Windows Background Service:** Architecture for unattended access under least-privilege IPC (`opensupport-service`).
- [x] **Automated Verification:** 100% passing test suite across all workspace crates.

---

## 🟡 In Active Development (v0.2.0-beta)

- [ ] **Windows Graphics Capture (WGC) & Desktop Duplication API:** Hardware pipeline integration with DirectX 11/12 frame capture.
- [ ] **NVENC / AMF / QuickSync Video Encoding:** H.264/AV1 hardware-accelerated video streaming.
- [ ] **Adaptive Network Engine:** Dynamic bitrate adaptation (250 Kbps - 20 Mbps) based on real-time RTT and packet loss detection.
- [ ] **MSI & WiX Toolset Enterprise Installer:** Signed MSI installer with silent deployment flags (`/quiet`) for Active Directory GPO and Microsoft Intune.

---

## 🔵 Planned Milestones (v1.0.0 Stable)

- [ ] **Secure Desktop & UAC Elevation Routing:** Handling credential prompts on Windows Secure Desktop without bypassing UAC.
- [ ] **Hardware Security Key MFA:** FIDO2 / WebAuthn and YubiKey support for technician authentication.
- [ ] **Remote Audio Redirection:** Low-latency bidirectional WASAPI audio streaming.
- [ ] **Air-Gapped Local Network Mode:** Direct LAN discovery (mDNS / UPnP) without external relay infrastructure.
- [ ] **Enterprise RBAC & Directory Sync:** LDAP / Microsoft Entra ID integration for role-based technician management.
