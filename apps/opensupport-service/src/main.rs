use std::ffi::OsString;
use std::time::Duration;
use tracing::{error, info};

#[cfg(windows)]
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
};

pub const SERVICE_NAME: &str = "OpenSupportAgent";
pub const SERVICE_DISPLAY_NAME: &str = "OpenSupport Remote Assistance Service";

#[cfg(windows)]
define_windows_service!(ffi_service_main, opensupport_service_main);

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    #[cfg(windows)]
    {
        // Si se ejecuta bajo el Service Control Manager (SCM)
        if let Err(e) = service_dispatcher::start(SERVICE_NAME, ffi_service_main) {
            info!("Iniciando en modo interactivo/consola: {:?}", e);
            run_service_worker()?;
        }
    }

    #[cfg(not(windows))]
    {
        run_service_worker()?;
    }

    Ok(())
}

#[cfg(windows)]
pub fn opensupport_service_main(arguments: Vec<OsString>) {
    if let Err(e) = run_service(arguments) {
        error!("Error en servicio Windows: {:?}", e);
    }
}

#[cfg(windows)]
fn run_service(_arguments: Vec<OsString>) -> anyhow::Result<()> {
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop | ServiceControl::Shutdown => {
                info!("Petición de parada recibida del SCM");
                ServiceControlHandlerResult::NoError
            }
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
        exit_code: ServiceExitCode::NO_ERROR,
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    run_service_worker()?;

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::NO_ERROR,
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    Ok(())
}

fn run_service_worker() -> anyhow::Result<()> {
    info!("OpenSupport Service Worker iniciado en Session 0 (SYSTEM). Supervisando sesiones y UAC...");
    Ok(())
}
