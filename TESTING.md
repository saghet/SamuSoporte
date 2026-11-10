# PLAN DE PRUEBAS Y VALIDACIÓN (TESTING.md)

> **Cobertura de Pruebas:** Criptografía E2EE, Integridad de Datos, Inyección UIPI, Codec y Rendimiento  

---

## 1. Pruebas Automatizadas del Motor Rust

Ejecución en consola:
```powershell
cargo test --workspace
```
- **`crypto_suite`**: Valida el protocolo Noise (`Noise_NN_25519_ChaChaPoly_BLAKE2s`), derivación de identidades y SAS anti-MitM.
- **`input_suite`**: Valida la creación de eventos de mouse y teclado y manejo de restricciones de integridad UIPI.
- **`transfer_suite`**: Valida el bloqueo de path traversal (`..`), filtrado de ejecutables y suma de verificación SHA-256 por bloques.
- **`diagnostics_suite`**: Valida la lectura de hardware, CPU, memoria y lista de procesos activos.
- **`audit_suite`**: Valida la detección inmediata de manipulación en la cadena de hashes del ledger de auditoría.
- **`codec_suite`**: Valida la compresión diferencial XOR y reconstrucción idéntica con LZ4.

---

## 2. Pruebas del Cliente Nativo Windows (.NET 10 & WinUI 3)

Ejecución de compilación y verificación de interfaz:
```powershell
dotnet test OpenSupport.slnx
```
- Navegación fluida por todas las pestañas de Fluent Design (`NavigationView`).
- Activación y desactivación del banner de sesión remota activa con botón de desconexión inmediata.
- Generación y copia del código temporal de 6 dígitos al portapapeles.
