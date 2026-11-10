# AUDITORÍA TÉCNICA E INVENTARIO DE COMPONENTES (FASE 0)

> **Proyecto:** OpenSupport Native Windows Migration  
> **Fecha:** 2026-11-09  
> **Estado:** Rama `feature/native-windows` activa | Rama `main` respaldada  

---

## 1. Inventario y Evaluación de Código Existente

| Crate / Módulo | Lenguaje / Tecnologías | Evaluación | Decisión de Migración |
| :--- | :--- | :--- | :--- |
| **`opensupport-crypto`** | Rust (snow, x25519, ed25519, chacha20poly1305) | **100% Reutilizable y Crítico.** Criptografía auditada E2EE y SAS anti-MitM. | Se exporta vía FFI (`cdylib` / C-ABI) para consumo directo desde C# / .NET 10 mediante P/Invoke seguro. |
| **`opensupport-input`** | Rust (Windows API `SendInput`, Virtual Desktop, UIPI) | **100% Reutilizable.** Lógica nativa de inyección multimonitor y normalización de coordenadas. | Se mantiene en Rust y se integra en el worker de entrada del servicio nativo. |
| **`opensupport-transfer`**| Rust (Async Tokio, SHA-256, Anti-Path Traversal) | **100% Reutilizable.** Motor de transferencia por chunks e integridad criptográfica. | Se expone vía librería dinámica compartida para transferencias concurrentes. |
| **`opensupport-codec`** | Rust (LZ4 Flex, XOR Diff Frame) | **100% Reutilizable.** Compresión y descompresión diferencial ultrarrápida. | Se invoca desde C# WinUI 3 para decodificación y renderizado directo sobre `WriteableBitmap` / DirectX SwapChain. |
| **`opensupport-diagnostics`**| Rust (sysinfo, Process, HW, Disk, Network) | **100% Reutilizable.** Diagnóstico multiplataforma sin depender de WMI lento. | Se consume directamente o se complementa con APIs nativas de Windows Internals en C#. |
| **`opensupport-audit`** | Rust (Tamper-evident Hash Chain, SHA-256 Ledger) | **100% Reutilizable.** Integridad inmutable de eventos. | Se preserva como motor de bitácora centralizado. |
| **`opensupport-server`**| Rust (Axum, Tokio, WebSockets, Zero-Knowledge Relay) | **100% Reutilizable.** Servidor relay y señalización autohospedado independiente de la UI. | Se mantiene intacto para despliegues Docker / On-Premise. |
| **`opensupport-service`**| Rust (Windows Service SCM, Session 0) | **Parcialmente Reutilizable.** | Se migra/refactoriza en dos capas: C# / .NET 10 Worker Service con elevación controlada + DLL Rust para inyección. |
| **`apps/opensupport-desktop`** | HTML/JS provisional | **COMPONENTE INCOMPATIBLE (ELIMINADO).** | Se descarta completamente conforme al requerimiento de aplicación de escritorio 100% nativa. |

---

## 2. Puntos de Fricción Identificados y Mitigaciones

1. **Interoperabilidad C# .NET 10 y Rust:**
   - *Riesgo:* Coste de cruzar la frontera P/Invoke si no se diseñan buffers contiguos.
   - *Mitigación:* Estructuras `[StructLayout(LayoutKind.Sequential)]`, strings UTF-8 terminados en null sin conversiones innecesarias, y callbacks no administrados pineados con `GCHandle`.
2. **Restricciones de Integridad de Windows (UIPI y UAC):**
   - *Riesgo:* Una aplicación WinUI estándar no puede interactuar con ventanas con privilegios elevados (UAC) o el Secure Desktop (`Winlogon`).
   - *Mitigación:* Arquitectura cliente-servicio desacoplada. La UI corre como usuario estándar; el servicio interactúa con la sesión activa mediante duplicación de tokens de seguridad sin saltarse UAC ni alterar la seguridad del SO.
3. **Framework de Interfaz de Usuario:**
   - *Elección:* **WinUI 3 (Windows App SDK)** sobre **.NET 10 LTS**, con controles nativos de Fluent Design, soporte completo de modo claro/oscuro, temas del sistema y aceleración de hardware nativa de Windows.
