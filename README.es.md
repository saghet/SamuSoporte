# SamuSoporte 🛡️ (Español)

[![CI / Build & Test](https://github.com/saghet/SamuSoporte/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/saghet/SamuSoporte/actions)
[![Security Policy](https://img.shields.io/badge/Security-Policy-blue.svg)](SECURITY.md)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPLv3-green.svg)](LICENSE)
[![.NET 10 LTS](https://img.shields.io/badge/.NET-10.0%20LTS-purple.svg)](https://dotnet.microsoft.com/)
[![Rust 2021](https://img.shields.io/badge/Rust-2021%20Stable-orange.svg)](https://www.rust-lang.org/)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2010%2F11-0078D4.svg)](https://www.microsoft.com/windows)

> **SamuSoporte** es una plataforma de asistencia técnica y control remoto de alto rendimiento, de código abierto y basada en arquitectura Zero Trust, desarrollada de forma nativa para Windows. Construida con **.NET 10**, **WPF-UI** y un **núcleo en Rust**, ofrece una alternativa profesional, auditable y segura frente a AnyDesk, TeamViewer y RustDesk.

[Read in English](README.md) | [Arquitectura](ARCHITECTURE.md) | [Hoja de Ruta](ROADMAP.md) | [Política de Seguridad](SECURITY.md)

---

## ⚡ Puntos Destacados

- **Aplicación 100% Nativa para Windows:** Desarrollada estrictamente en C# (.NET 10 LTS) y WPF-UI con efectos visuales Mica/Acrylic acelerados por hardware. **Sin Electron, sin navegadores web incrustados, sin wrappers HTML.**
- **Arquitectura Zero Trust y Zero-Knowledge:** Implementa el estándar **Noise Protocol Framework** (`Noise_XX_25519_ChaChaPoly_BLAKE2s`). Todo el tráfico viaja cifrado de extremo a extremo (E2EE) entre los equipos; el servidor relay intermedio sólo enruta paquetes ciegos y no puede descifrar ni inspeccionar sesiones.
- **Autenticación Mutua Anti-MitM (SAS):** Verificación numérica Short Authentication String (SAS) para eliminar ataques de intermediario y suplantación.
- **Higiene Criptográfica en Memoria:** Claves privadas efímeras y secretos de sesión utilizan `Zeroize` para destruirse de la memoria RAM inmediatamente al descartarse.
- **Bitácora de Auditoría Inmutable:** Registro forense encadenado con firmas criptográficas SHA-256 a prueba de manipulaciones, exportable a sistemas SIEM (JSON-L).
- **Servicio Windows de Mínimo Privilegio:** Separación por servicios para conexiones desatendidas y soporte en el Escritorio Seguro sin evasión insegura de UAC.

---

## 🖥️ Módulos e Interfaz de Usuario

SamuSoporte incluye una interfaz completa y fluida:
1. **Panel Principal:** Generación de código temporal de 6 dígitos, conexión directa a equipos remotos y botón de **Parada de Emergencia** inmediata.
2. **Soporte Remoto:** Visor de baja latencia con conmutación multi-monitor, sincronización de portapapeles y señal SAS `Ctrl+Alt+Del`.
3. **Mis Dispositivos:** Inventario de equipos registrados con estado en tiempo real y soporte para paquetes mágicos Wake-on-LAN (WoL).
4. **Consola del Técnico:** Monitoreo en vivo de telemetría de hardware (CPU, RAM, disco NVMe) y herramientas directas de diagnóstico.
5. **Seguridad & Zero Trust:** Visualización del túnel Noise activo, huellas digitales de certificados y políticas de acceso.
6. **Auditoría Inmutable:** Visor cronológico de sucesos de seguridad verificables con hash criptográfico.
7. **Configuración:** Ajustes de conexión a servidores relay privados, Certificate Pinning y opciones de aceleración por GPU.

---

## 🚀 Requisitos y Compilación

### Requisitos Previos
- Windows 10 (Compilación 19041+) o Windows 11
- [.NET 10 SDK](https://dotnet.microsoft.com/download/dotnet/10.0)
- [Rust Stable](https://rustup.rs/) (`x86_64-pc-windows-msvc`)
- Visual Studio 2022 / Herramientas de compilación C++ (Windows SDK 10.0.26100.0)

### 1. Compilación del Cliente de Escritorio (.NET 10)
```powershell
dotnet build src/OpenSupport.Desktop/OpenSupport.Desktop.csproj -c Release
```

Para generar un ejecutable único autocontenido:
```powershell
dotnet publish src/OpenSupport.Desktop/OpenSupport.Desktop.csproj -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true -o publish/desktop
```

### 2. Compilación del Núcleo Rust y Servidor Relay
```powershell
cargo build --workspace --release
```

### 3. Ejecución de Pruebas Automatizadas
```powershell
# Pruebas unitarias de Rust (Criptografía, Codec, Transferencia, Auditoría, Diagnósticos)
cargo test --workspace

# Pruebas de la solución .NET
dotnet test
```

---

## 📄 Licencia

Este proyecto está distribuido bajo la licencia **GNU Affero General Public License v3.0 (AGPL-3.0)** - consulta el archivo [LICENSE](LICENSE) para más detalles. Para integraciones comerciales que requieran código privativo, consulta por nuestras opciones de doble licenciamiento.
