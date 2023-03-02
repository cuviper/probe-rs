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
