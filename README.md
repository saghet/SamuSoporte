# SamuSoporte 🛡️

[![CI / Build & Test](https://github.com/saghet/SamuSoporte/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/saghet/SamuSoporte/actions)
[![Security Policy](https://img.shields.io/badge/Security-Policy-blue.svg)](SECURITY.md)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPLv3-green.svg)](LICENSE)
[![.NET 10 LTS](https://img.shields.io/badge/.NET-10.0%20LTS-purple.svg)](https://dotnet.microsoft.com/)
[![Rust 2021](https://img.shields.io/badge/Rust-2021%20Stable-orange.svg)](https://www.rust-lang.org/)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2010%2F11-0078D4.svg)](https://www.microsoft.com/windows)

> **SamuSoporte** is a high-performance, open-source, Zero Trust remote assistance and desktop control platform engineered natively for Windows. Built with **.NET 10**, **WPF-UI**, and a **Rust core**, it delivers an uncompromising, auditable alternative to AnyDesk, TeamViewer, and RustDesk.

[📖 Leer en Español](README.es.md) | [Architecture](ARCHITECTURE.md) | [Roadmap](ROADMAP.md) | [Security Policy](SECURITY.md)

---

## ⚡ Key Highlights

- **100% Native Windows Experience:** Engineered strictly in C# (.NET 10 LTS) and WPF-UI with hardware-accelerated Mica/Acrylic composition. **No Electron, no embedded web browsers, no HTML wrappers.**
- **Zero Trust & Zero-Knowledge Architecture:** Powered by the **Noise Protocol Framework** (`Noise_XX_25519_ChaChaPoly_BLAKE2s`). All traffic is end-to-end encrypted (E2EE) on the endpoints; intermediate relay servers route opaque packets and cannot inspect or intercept sessions.
- **Anti-MitM Verification (SAS):** Mutual numeric Short Authentication String (SAS) verification eliminates Man-in-the-Middle and spoofing attacks.
- **In-Memory Cryptographic Hygiene:** Ephemeral private keys and session keys implement `Zeroize` to scrub sensitive data from RAM upon disposal.
- **Tamper-Evident Forensic Audit Trail:** Security events are cryptographically enciphered and hashed into an immutable SHA-256 chain, ready for SIEM (JSON-L) export.
- **Least-Privilege Windows Service:** Dedicated background service separation for unattended access and Secure Desktop interactions without dangerous UAC bypasses.

---

## 🖥️ User Interface & Modules

SamuSoporte provides a comprehensive, responsive workstation interface:
1. **Panel Principal (Main Dashboard):** Generates ephemeral 6-digit support codes, manages direct peer connections, and features an instant **Emergency Stop** killswitch.
2. **Soporte Remoto (Remote Control):** Low-latency screen viewer with multi-monitor switching, bidirectional clipboard sync, and secure SAS `Ctrl+Alt+Del` elevation.
3. **Mis Dispositivos (Managed Devices):** Real-time device catalog with online/offline statuses and Wake-on-LAN (WoL) triggers.
4. **Consola del Técnico (Technician Console):** Live remote hardware telemetry (CPU, RAM, NVMe) and direct administrative diagnostics.
5. **Seguridad & Zero Trust:** Real-time visibility into active Noise tunnels, public key fingerprints, and Zero Trust access policies.
6. **Auditoría Inmutable (Immutable Audit):** Cryptographically verifiable chronological log viewer.
7. **Configuración (Settings):** Relay server rendezvous configuration, certificate pinning, and GPU acceleration toggles.

---

## 🏗️ Repository Architecture

```
SamuSoporte/
├── apps/
│   ├── opensupport-server/       # High-throughput Rust relay & rendezvous server
│   └── opensupport-service/      # Rust Windows service orchestrator
├── crates/
│   ├── opensupport-core/         # Protocol messages, framing & data schemas
│   ├── opensupport-crypto/       # Noise Protocol E2EE, Ed25519, SAS, Zeroize
│   ├── opensupport-codec/        # Differential keyframes & compression
│   ├── opensupport-transfer/     # Resilient chunked file transfer with SHA-256
│   ├── opensupport-input/        # Windows SendInput mouse/keyboard driver
│   ├── opensupport-diagnostics/  # Hardware, memory & process telemetry
│   └── opensupport-audit/        # Tamper-evident SHA-256 hash-chained log
├── src/
│   ├── OpenSupport.Desktop/      # Native WPF-UI client (.NET 10 LTS)
│   └── OpenSupport.Service/      # .NET Windows background service worker
├── .github/                      # CI/CD Workflows & Issue Templates
├── ARCHITECTURE.md               # Detailed architectural specification
├── ROADMAP.md                    # Project development roadmap
└── SECURITY.md                   # Vulnerability disclosure policy
```

---

## 🚀 Getting Started & Compilation

### Requirements
- Windows 10 (Build 19041+) or Windows 11
- [.NET 10 SDK](https://dotnet.microsoft.com/download/dotnet/10.0)
- [Rust Stable](https://rustup.rs/) (`x86_64-pc-windows-msvc`)
- Visual Studio 2022 / C++ Build Tools (Windows SDK 10.0.26100.0)

### 1. Build the Native Desktop Client
```powershell
dotnet build src/OpenSupport.Desktop/OpenSupport.Desktop.csproj -c Release
```

To generate a single-file self-contained executable:
```powershell
dotnet publish src/OpenSupport.Desktop/OpenSupport.Desktop.csproj -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true -o publish/desktop
```

### 2. Build the Rust Core & Relay Server
```powershell
cargo build --workspace --release
```

### 3. Run Automated Tests
```powershell
# Run the Rust test suite (Crypto, Codec, Transfer, Audit, Diagnostics)
cargo test --workspace

# Run .NET solution tests
dotnet test
```

---

## 🔒 Security & Vulnerability Reporting

Please review our [Security Policy](SECURITY.md) for vulnerability disclosure guidelines. **Do not submit security flaws via public GitHub issues.**

---

## 📄 License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)** - see the [LICENSE](LICENSE) file for details. Commercial dual-licensing options are available for enterprise environments requiring proprietary integration.
