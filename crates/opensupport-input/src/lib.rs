pub mod error;
#[cfg(windows)]
pub mod windows;

pub use error::InputError;

#[cfg(windows)]
pub use windows::WindowsInputInjector as InputInjector;
