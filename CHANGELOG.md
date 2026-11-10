# REGISTRO DE CAMBIOS (CHANGELOG.md)

Todas las modificaciones notables de OpenSupport se documentan en este archivo.

## [0.2.0-native-alpha] - 2026-11-09

### Agregado
- **Migración a Aplicación Nativa Windows:**
  - Nueva solución `OpenSupport.slnx` con .NET 10 LTS y Windows App SDK.
  - Implementación de `OpenSupport.App` con **WinUI 3**, material *Mica*, modo oscuro/claro y controles nativos de Fluent Design.
  - Creación de vistas nativas completas:
    - `HomePage`: Generador de código temporal asistido de 6 dígitos, conexión a target y **Banner permanente de sesión activa** con parada de emergencia.
    - `RemoteSupportPage`: Superficie de renderizado de pantalla remota y CommandBar de control.
    - `DevicesPage`: Gestión de equipos registrados y estados en línea.
    - `TechnicianConsolePage`: Caja de herramientas de telemetría de CPU/RAM/Disco y administrador de procesos remoto.
    - `SecurityPage`: Configuración Zero Trust, clave pública Ed25519 y opciones de endurecimiento.
    - `AuditPage`: Explorador del registro criptográfico inmutable de eventos con validación de hashes.
  - Creación de `OpenSupport.Service`: Servicio nativo para Windows (`Microsoft.Extensions.Hosting.WindowsServices`) con servidor IPC de Named Pipes y descriptores de seguridad SDDL.
- **Acceso Directo en Escritorio:**
  - Creación del acceso directo oficial `OpenSupport.lnk` apuntando al binario nativo compilado `OpenSupport.App.exe`.
- **Documentación Técnica de Ingeniería:**
  - `AUDIT_FASE0.md`, `ARCHITECTURE.md`, `THREAT_MODEL.md`, `PROJECT_STATUS.md`, `BUILD.md`, `INSTALLATION.md`, `TESTING.md`.

### Modificado
- Integración del motor criptográfico, streaming diferencial y relay desarrollados en Rust para consumo interoperable mediante FFI.
