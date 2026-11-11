# SamuSoporte System Architecture

SamuSoporte is a high-performance, open-source, Zero Trust remote support platform engineered natively for Windows. It provides a secure, fully auditable alternative to commercial proprietary tools (AnyDesk, TeamViewer, RustDesk).

---

## 1. High-Level Architectural Diagram

```
+---------------------------------------------------------------------------------+
|                         CLIENT MACHINE (WINDOWS 10/11)                          |
|                                                                                 |
|  +---------------------------------------------------------------------------+  |
|  |                 SamuSoporte Native Desktop App (.NET 10)                 |  |
|  |   - WPF-UI Fluent Design (Mica, Acrylic)                                 |  |
|  |   - Interactive Views (Remote Screen, Devices, Tech Console, Security)    |  |
|  +-------------------------------------+-------------------------------------+  |
|                                        |                                        |
|                       Internal IPC / Named Pipe                                 |
|                                        v                                        |
|  +---------------------------------------------------------------------------+  |
|  |                   Rust Core Engine (opensupport-core)                     |  |
|  |   - opensupport-crypto: Noise Protocol E2EE, Ed25519, SAS Anti-MitM      |  |
|  |   - opensupport-codec: Frame diffs, RLE, Keyframe reconstruction          |  |
|  |   - opensupport-transfer: Chunked file transfer, SHA-256 integrity checks |  |
|  |   - opensupport-input: Windows SendInput, mouse & keyboard translation    |  |
|  |   - opensupport-audit: SHA-256 hash-chained forensic audit trail          |  |
|  |   - opensupport-diagnostics: CPU, RAM, NVMe, Windows process enumerator   |  |
|  +-------------------------------------+-------------------------------------+  |
+----------------------------------------|----------------------------------------+
                                         |
             Encrypted E2EE Tunnel (TLS 1.3 / WebSocket Transport)
             Noise_XX_25519_ChaChaPoly_BLAKE2s
                                         |
                                         v
+---------------------------------------------------------------------------------+
|                       SELF-HOSTED RELAY SERVER (RUST)                           |
|                                                                                 |
|  +---------------------------------------------------------------------------+  |
|  |                   opensupport-server (Axum + Tokio)                       |  |
|  |   - Zero-Knowledge Packet Router (Cannot decrypt or inspect payload)      |  |
|  |   - Rendezvous Signaling & Session Negotiation Engine                     |  |
|  |   - Rate Limiting, Handshake Authentication & Anti-Replay                 |  |
|  +---------------------------------------------------------------------------+  |
+---------------------------------------------------------------------------------+
```

---

## 2. Component Breakdown

### A. SamuSoporte.Desktop (`src/OpenSupport.Desktop`)
- **Technology:** C# with **.NET 10 LTS** and **WPF-UI** (Windows Presentation Foundation Fluent System).
- **Design Philosophy:** 100% Native Windows binary. No Electron, no Chromium Embedded Framework, zero browser engine overhead.
- **Backdrop & Windowing:** Leverages Windows 11 `Mica` and `Acrylic` composition effects via DWM APIs.
- **Reactive Navigation:** Single-window reactive view switcher with 6 modular workstations.

### B. Windows Service (`apps/opensupport-service` & `src/OpenSupport.Service`)
- Runs as an independent Windows service under least-privilege service accounts (`NT AUTHORITY\LocalService` or dedicated service SID).
- Manages unattended access, device authorization verification, and SAS signal injection (`Ctrl+Alt+Del`).
- Hardened Named Pipe IPC with strict ACLs and caller PID validation.

### C. Rust Core Engine (`crates/`)
- **`opensupport-crypto`**: Full implementation of Noise Protocol Framework (`Noise_XX_25519_ChaChaPoly_BLAKE2s`). Manages Ed25519 device signatures, ChaCha20-Poly1305 symmetric AEAD encryption, and Short Authentication String (SAS) calculation. Implements `Zeroize` for automatic memory sanitization.
- **`opensupport-codec`**: Differential frame processing, bounding box change detection, and keyframe compression.
- **`opensupport-transfer`**: Resilient chunk-based file transfer with strict path traversal validation (`..` and directory escaping prevention) and SHA-256 cryptographic hashes.
- **`opensupport-audit`**: Tamper-evident, forward-secure forensic log chain. Each audit entry contains the SHA-256 digest of the preceding entry.
- **`opensupport-input`**: Native Windows input injection using `SendInput` APIs with relative coordinate normalization.
- **`opensupport-diagnostics`**: Real-time hardware and process monitoring using Windows performance APIs (`sysinfo`).

### D. Zero-Knowledge Relay Server (`apps/opensupport-server`)
- Asynchronous high-throughput server written in Rust (`tokio` + `axum` + `tokio-tungstenite`).
- Operates as a blind packet broker: only parses signaling metadata (target device ID) and relays opaque E2EE payloads. Even in the event of relay compromise, session confidentiality remains mathematically unbreakable.
