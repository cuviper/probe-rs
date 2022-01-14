#[doc(hidden)]
#[macro_export]
macro_rules! platform_probe(
    ($provider:ident, $name:ident, $($arg:expr,)*) => ()
);
