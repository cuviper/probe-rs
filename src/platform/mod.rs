#[cfg(any(target_os = "linux", target_os = "android"))]
mod systemtap;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub use systemtap::*;

#[cfg(not(any(target_os = "linux", target_os = "android")))]
mod default;
#[cfg(not(any(target_os = "linux", target_os = "android")))]
pub use default::*;
