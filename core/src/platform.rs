#[cfg(not(windows))]
mod mock;
#[cfg(windows)]
mod windows;

#[cfg(not(windows))]
pub use mock::MockScanner as PlatformScanner;
#[cfg(windows)]
pub use windows::WindowsScanner as PlatformScanner;
