# GUÍA DE INSTALACIÓN Y DESPLIEGUE (INSTALLATION.md)

> **OpenSupport:** Plataforma Nativa de Asistencia Remota Segura  

---

## 1. Requisitos del Sistema
- **Sistema Operativo:** Windows 10 (Build 17763 o superior), Windows 11 o Windows Server 2019/2022/2025.
- **Arquitectura:** x64 / ARM64.
- **Runtimes:** .NET Desktop Runtime 10.0 y Windows App SDK Runtime (incluido en el paquete).

---

## 2. Instalación de la Aplicación y Servicio

### Acceso Directo de Escritorio
El instalador o script de inicialización crea un acceso directo nativo `OpenSupport.lnk` en el Escritorio que enlaza directamente a:
`src\OpenSupport.App\bin\Release\net10.0-windows10.0.26100.0\win-x64\OpenSupport.App.exe`

### Registro del Servicio de Windows (Administrador)
Para habilitar el soporte desatendido supervisado y control en pantallas UAC:
```cmd
sc create OpenSupportAgent binPath= "C:\RutaInstalacion\OpenSupport.Service.exe" start= auto
sc start OpenSupportAgent
```
