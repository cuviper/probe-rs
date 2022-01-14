#[cfg(any(target_os = "linux", target_os = "android"))]
mod systemtap;

#[cfg(not(any(target_os = "linux", target_os = "android")))]
mod default;
