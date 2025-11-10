#[cfg(any(target_os = "linux", target_os = "android", docsrs))]
mod systemtap;
#[cfg(any(target_os = "linux", target_os = "android", docsrs))]
pub use systemtap::*;

#[cfg(not(any(target_os = "linux", target_os = "android", docsrs)))]
mod default;
#[cfg(not(any(target_os = "linux", target_os = "android", docsrs)))]
pub use default::*;
