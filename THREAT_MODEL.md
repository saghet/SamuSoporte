# MODELO DE AMENAZAS (STRIDE) - OPENSUPPORT NATIVO WINDOWS

> **Evaluación de Seguridad:** Arquitectura de Aplicación Nativa Windows (.NET 10 + WinUI 3 + Rust)  
> **Metodología:** Microsoft STRIDE Threat Modeling  

---

## 1. Matriz de Análisis de Amenazas STRIDE

| Categoría | Amenaza Específica | Vector de Ataque | Mitigación Implementada en OpenSupport |
| :--- | :--- | :--- | :--- |
| **Spoofing** | Suplantación de la identidad del técnico o del equipo remoto. | Conexión fraudulenta fingiendo tener la clave pública del técnico. | Identidades criptográficas Ed25519 con verificación mutua en cada conexión y visualización de **Short Authentication String (SAS)** en pantalla. |
| **Tampering** | Inyección de paquetes maliciosos o comandos en tránsito; manipulación de archivos en memoria. | Modificación de eventos de entrada o manipulación del buffer de transferencia. | Cifrado autenticado AEAD (ChaCha20-Poly1305) en cada paquete; hash SHA-256 por bloques verificado antes de escribir en disco. |
| **Repudiation** | El técnico realiza una acción perjudicial y niega haberla hecho. | Eliminación o modificación de logs locales de auditoría. | **Cadena de hashes SHA-256 encadenada inmutable (Audit Ledger)**; cada entrada incluye el hash de la anterior. |
| **Information Disclosure** | Interceptación de imágenes de pantalla o credenciales en el relay. | Servidor Relay interceptando tráfico entre ambos extremos. | **Zero-Knowledge Relay:** El relay sólo retransmite payloads cifrados. No posee claves de sesión (PFS con Noise Protocol). |
| **Denial of Service** | Ataques de flooding de conexiones en el Named Pipe local o sockets de red. | Un proceso sin privilegios agota los descriptores de conexión del servicio. | Named Pipe protegido por ACLs SDDL restrictivas; rate limiting estricto en el servidor de señalización. |
| **Elevation of Privilege** | Un usuario estándar utiliza el servicio Windows (SYSTEM) para ejecutar código arbitrario (Privilege Escalation). | Manipulación del canal IPC para forzar al servicio a lanzar ejecutables no autorizados. | El servicio no acepta rutas de ejecutables arbitrarios por IPC. Sólo procesa tipos estructurados y estáticos de eventos de entrada (`SendInput`). |

---

## 2. Protección de Secretos y Credenciales en Windows

- **Protección de Claves Privadas:** Las claves de identidad del equipo se cifran en reposo utilizando **DPAPI (Data Protection API)** a través de `ProtectedData.Protect` con ámbito `DataProtectionScope.CurrentUser` o `LocalMachine` según la modalidad.
- **Sin exclusiones en Windows Defender:** La aplicación no añade reglas de exclusión en Windows Defender ni manipula las directivas de seguridad locales.
