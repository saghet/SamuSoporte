#[cfg(test)]
mod tests {
    use opensupport_diagnostics::DiagnosticsCollector;

    #[test]
    fn test_diagnostics_collector_system_overview() {
        let mut collector = DiagnosticsCollector::new();
        let overview = collector.get_system_overview();

        assert!(!overview.os_name.is_empty(), "OS Name no debe estar vacío");
        assert!(overview.cpu_count > 0, "CPU Count debe ser mayor a 0");
        assert!(overview.total_memory_bytes > 0, "Total RAM debe ser mayor a 0");
    }

    #[test]
    fn test_diagnostics_collector_processes() {
        let mut collector = DiagnosticsCollector::new();
        let procs = collector.list_processes();

        assert!(!procs.is_empty(), "Lista de procesos no debe estar vacía");
        let current_pid = std::process::id();
        let found_self = procs.iter().any(|p| p.pid == current_pid);
        assert!(found_self, "El proceso actual debe aparecer en la lista");
    }
}
