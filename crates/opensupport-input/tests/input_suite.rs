#[cfg(test)]
mod tests {
    use opensupport_core::protocol::InputEvent;
    use opensupport_input::{InputError, InputInjector};

    #[test]
    fn test_input_injector_instantiation_and_structure() {
        let injector = InputInjector::new();
        // Comprobamos la inyección sintética de eventos en el entorno de pruebas
        let move_event = InputEvent::MouseMove { x: 100, y: 100 };
        let res = injector.inject(&move_event);
        
        // En entornos de CI o procesos sin permisos UIPI / Session 0, Windows protege el escritorio
        // devolviendo E_ACCESSDENIED. El inyector debe manejarlo limpiamente como InputError::PermissionDenied.
        match res {
            Ok(()) => {}
            Err(InputError::PermissionDenied(msg)) => {
                println!("Entorno protegido por UIPI / Desktop Access: {}", msg);
            }
            Err(e) => panic!("Error inesperado en SendInput: {:?}", e),
        }
    }
}
