use crate::error::TransferError;
use std::path::{Component, Path, PathBuf};

pub struct PathSecurityValidator {
    allowed_base_dir: PathBuf,
    blocked_extensions: Vec<String>,
    max_file_size_bytes: u64,
}

impl PathSecurityValidator {
    pub fn new(allowed_base_dir: PathBuf, max_file_size_bytes: u64) -> Self {
        let blocked = vec![
            "exe".into(), "bat".into(), "cmd".into(), "vbs".into(),
            "ps1".into(), "scr".into(), "pif".into(), "com".into(),
            "msi".into(), "reg".into(), "dll".into(),
        ];

        Self {
            allowed_base_dir,
            blocked_extensions: blocked,
            max_file_size_bytes,
        }
    }

    pub fn with_custom_blocked_extensions(mut self, extensions: Vec<String>) -> Self {
        self.blocked_extensions = extensions;
        self
    }

    /// Valida que el nombre de archivo no contenga secuencias de escape de directorio
    /// y resuelve una ruta estrictamente confinada dentro de `allowed_base_dir`.
    pub fn sanitize_and_resolve(&self, filename: &str, file_size: u64) -> Result<PathBuf, TransferError> {
        if file_size > self.max_file_size_bytes {
            return Err(TransferError::FileTooLarge(self.max_file_size_bytes));
        }

        let path = Path::new(filename);

        // Prevenir Path Traversal inspeccionando componentes
        for component in path.components() {
            match component {
                Component::ParentDir => {
                    return Err(TransferError::SecurityViolation(format!(
                        "Secuencia de escape '..' detectada en: {}",
                        filename
                    )));
                }
                Component::RootDir | Component::Prefix(_) => {
                    return Err(TransferError::SecurityViolation(format!(
                        "Ruta absoluta no permitida en nombre relativo: {}",
                        filename
                    )));
                }
                _ => {}
            }
        }

        // Extraer nombre de archivo base seguro
        let file_name = path
            .file_name()
            .ok_or_else(|| TransferError::SecurityViolation("Nombre de archivo vacío".into()))?
            .to_string_lossy();

        // Validar extensión prohibida
        if let Some(ext) = Path::new(&*file_name).extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if self.blocked_extensions.iter().any(|b| b.to_lowercase() == ext_str) {
                return Err(TransferError::BlockedExtension(ext_str));
            }
        }

        let resolved = self.allowed_base_dir.join(&*file_name);
        Ok(resolved)
    }
}
