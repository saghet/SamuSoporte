# OpenSupport: Guía de Compilación (BUILD.md)

> **Plataforma:** Windows 10 / Windows 11 / Windows Server  
> **Requisitos:** .NET 10 SDK (v10.0.403+) y Rust Stable (v1.80+)  

---

## 1. Estructura de la Solución

- **`OpenSupport.slnx`**: Solución principal de .NET 10.
  - **`src/OpenSupport.App`**: Aplicación de escritorio nativa WinUI 3 (Windows App SDK) con Fluent Design y NavigationView.
  - **`src/OpenSupport.Service`**: Servicio de Windows en Session 0 con IPC Named Pipe seguro para elevación y gestión UAC.
- **`Cargo.toml`**: Monorepo en Rust para el núcleo criptográfico (`opensupport-crypto`), codec diferencial (`opensupport-codec`), motor de entrada (`opensupport-input`), transferencia de archivos (`opensupport-transfer`), auditoría (`opensupport-audit`) y servidor relay (`apps/opensupport-server`).

---

## 2. Instrucciones de Compilación

### Compilación de la Aplicación Nativa Windows (.NET 10 & WinUI 3)
```powershell
dotnet build src/OpenSupport.App/OpenSupport.App.csproj -c Release
```
El ejecutable binario nativo se genera en:
`src/OpenSupport.App/bin/Release/net10.0-windows10.0.26100.0/win-x64/OpenSupport.App.exe`

### Compilación del Servicio de Windows (.NET 10)
```powershell
dotnet build src/OpenSupport.Service/OpenSupport.Service.csproj -c Release
```

### Compilación del Servidor Relay y Criptografía (Rust)
```powershell
cargo build --release --bin opensupport-server
```
