use opensupport_core::protocol::ProcessSummary;
use sysinfo::{Disks, Networks, Pid, System};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiagnosticError {
    #[error("Proceso con PID {0} no encontrado")]
    ProcessNotFound(u32),

    #[error("No se pudo terminar el proceso con PID {0}")]
    KillFailed(u32),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HostSystemOverview {
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
    pub cpu_count: usize,
    pub global_cpu_usage_pct: f32,
    pub total_memory_bytes: u64,
    pub used_memory_bytes: u64,
    pub total_swap_bytes: u64,
    pub used_swap_bytes: u64,
    pub disks: Vec<DiskOverview>,
    pub network_interfaces: Vec<NetworkOverview>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiskOverview {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkOverview {
    pub interface_name: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
}

pub struct DiagnosticsCollector {
    sys: System,
    disks: Disks,
    networks: Networks,
}

impl DiagnosticsCollector {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();

        Self {
            sys,
            disks,
            networks,
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
        self.disks.refresh();
        self.networks.refresh();
    }

    pub fn get_system_overview(&mut self) -> HostSystemOverview {
        self.refresh();

        let os_name = System::name().unwrap_or_else(|| "Unknown".into());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".into());
        let hostname = System::host_name().unwrap_or_else(|| "Unknown".into());

        let disks_summary = self
            .disks
            .iter()
            .map(|d| DiskOverview {
                name: d.name().to_string_lossy().to_string(),
                mount_point: d.mount_point().to_string_lossy().to_string(),
                total_space: d.total_space(),
                available_space: d.available_space(),
            })
            .collect();

        let net_summary = self
            .networks
            .iter()
            .map(|(name, data)| NetworkOverview {
                interface_name: name.clone(),
                received_bytes: data.total_received(),
                transmitted_bytes: data.total_transmitted(),
            })
            .collect();

        HostSystemOverview {
            os_name,
            os_version,
            hostname,
            cpu_count: self.sys.cpus().len(),
            global_cpu_usage_pct: self.sys.global_cpu_usage(),
            total_memory_bytes: self.sys.total_memory(),
            used_memory_bytes: self.sys.used_memory(),
            total_swap_bytes: self.sys.total_swap(),
            used_swap_bytes: self.sys.used_swap(),
            disks: disks_summary,
            network_interfaces: net_summary,
        }
    }

    pub fn list_processes(&mut self) -> Vec<ProcessSummary> {
        self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All);

        self.sys
            .processes()
            .iter()
            .map(|(pid, proc)| ProcessSummary {
                pid: pid.as_u32(),
                name: proc.name().to_string_lossy().to_string(),
                cpu_pct: proc.cpu_usage(),
                memory_bytes: proc.memory(),
            })
            .collect()
    }

    pub fn kill_process(&mut self, target_pid: u32) -> Result<(), DiagnosticError> {
        self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All);
        let pid = Pid::from_u32(target_pid);

        if let Some(process) = self.sys.process(pid) {
            if process.kill() {
                Ok(())
            } else {
                Err(DiagnosticError::KillFailed(target_pid))
            }
        } else {
            Err(DiagnosticError::ProcessNotFound(target_pid))
        }
    }
}

impl Default for DiagnosticsCollector {
    fn default() -> Self {
        Self::new()
    }
}
