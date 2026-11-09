use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Windows API Error: {0}")]
    WindowsApi(String),

    #[error("Permission Denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid Input coordinates ({0}, {1})")]
    InvalidCoordinates(i32, i32),
}
