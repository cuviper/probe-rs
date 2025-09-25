#[repr(transparent)]
pub struct Semaphore;

impl Semaphore {
    /// Return a `Semaphore` that starts as disabled.
    pub const fn new() -> self {
        Self
    }

    /// Return whether a debugger or tracing tool is attached to a probe
    /// that uses this semaphore.
    pub fn enabled(&self) -> bool {
        false
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! platform_probe(
    ($provider:ident, $name:ident, $($arg:expr,)*) => ({
        // Non-lazy probes always evaluate the arguments.
        let _ = ($($arg,)*);
    })
);

#[doc(hidden)]
#[macro_export]
macro_rules! platform_probe_lazy(
    ($provider:ident, $name:ident, $($arg:expr,)*) => ({
        // Expand the arguments so they don't cause unused warnings.
        if false {
            let _ = ($($arg,)*);
        }
        false
    })
);
