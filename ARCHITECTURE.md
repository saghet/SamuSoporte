# ARQUITECTURA TÉCNICA DE LA APLICACIÓN NATIVA WINDOWS

> **Estándar:** .NET 10 LTS + WinUI 3 (Windows App SDK) + Rust FFI Engine + Windows Services  
> **Patrón Arquitectónico:** Clean Architecture + MVVM + Microservicios Locales IPC  

---

## 1. Diagrama de Capas del Sistema Nativo

```mermaid
flowchart TB
    subgraph UI_Layer["Capa de Presentación Nativa (WinUI 3 - C# / .NET 10)"]
        MainWindow["MainWindow (Fluent Window & Mica / Acrylic)"]
        NavView["NavigationView (Host, Técnico, Mis Dispositivos, Auditoría, Config)"]
        ViewModels["MVVM ViewModels (CommunityToolkit.Mvvm)"]
        DXViewer["DirectX / Win2D Remote Screen Surface"]
        MainWindow --> NavView
        NavView --> ViewModels
        ViewModels --> DXViewer
    end

    subgraph CoreBridge["Capa de Interoperabilidad (P/Invoke & C-ABI)"]
        InteropBridge["OpenSupport.Interop (P/Invoke seguro y marshaling con GCHandle)"]
        ViewModels --> InteropBridge
    end

    subgraph RustEngine["Motor de Comunicaciones y Criptografía (Rust cdylib)"]
        CryptoCore["opensupport_crypto (Noise E2EE, X25519, Ed25519, SAS)"]
        CodecCore["opensupport_codec (XOR Frame Diff, LZ4 Streaming)"]
        TransferCore["opensupport_transfer (Asynchronous Chunk Streaming, SHA-256)"]
        DiagnosticsCore["opensupport_diagnostics (Sysinfo Nativo)"]
        InteropBridge <--> CryptoCore
        InteropBridge <--> CodecCore
        InteropBridge <--> TransferCore
        InteropBridge <--> DiagnosticsCore
    end

    subgraph ServiceLayer["Servicio de Windows (Session 0 / LocalSystem - C# & Rust)"]
        WinService["OpenSupport.Service (Worker Service .NET 10)"]
        IPCServer["Servidor IPC Named Pipe Seguro (SDDL / ACLs estrictas)"]
        InputWorker["Worker de Inyección Elevado (SendInput & UAC Aware)"]
        WinService --> IPCServer
        WinService --> InputWorker
    end

    subgraph RemoteInfra["Servidores de Red (On-Premise / Docker / Pública)"]
        RelayServer["OpenSupport Relay & Rendezvous Server (Axum / Rust)"]
    end

    InteropBridge <-->|IPC Named Pipe Seguro| IPCServer
    RustEngine <===>|WebSockets / QUIC E2EE| RelayServer
```

---

## 2. Especificación de Componentes

### A. Aplicación de Escritorio WinUI 3 (`OpenSupport.App`)
- Desarrollada en C# y .NET 10 con soporte nativo de **Windows App SDK**.
- **Sin navegadores, sin WebView2 embebido como UI, sin HTML/CSS.**
- Controles 100% nativos: `NavigationView`, `ContentDialog`, `InfoBar`, `DataGrid`, `NumberBox`.
- Compatibilidad con diseño Fluent: Efectos de material *Mica*, bordes redondeados y alternancia dinámica de temas claro/oscuro.

### B. Puente FFI Nativo (`opensupport-ffi`)
- Crate en Rust configurado como `crate-type = ["cdylib", "rlib"]`.
- Expone funciones C-ABI `extern "C"` con convención `stdcall` / `cdecl`:
  - `opensupport_crypto_generate_identity()`
  - `opensupport_crypto_init_handshake()`
  - `opensupport_codec_compress_diff()`
  - `opensupport_codec_decompress_diff()`
  - `opensupport_diagnostics_get_overview()`

### C. Servicio de Windows (`OpenSupport.WindowsService`)
- Implementado como un servicio Windows de .NET 10 (`Microsoft.Extensions.Hosting.WindowsServices`).
- Escucha peticiones de la interfaz local a través de un **Named Pipe con Descriptores de Seguridad (SDDL)** que restringe el acceso únicamente al usuario de la sesión interactiva y al administrador.
- Ejecuta inyección de teclado y mouse incluso en pantallas protegidas por UAC sin comprometer la política de seguridad del SO.
