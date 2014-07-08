# Static instrumentation probe! macro for Rust

With the `probe!` macro, programmers can place static instrumentation
points in their code to mark events of interest.  These are compiled into
platform-specific implementations, e.g. SystemTap SDT on Linux.  Probes are
designed to have negligible overhead during normal operation, so they can
be present in all builds, and only activated using those external tools.

## Documentation

TODO

## Building libprobe

Simply run `rustc src/lib.rs` and copy the libprobe shared object to an
appropriate library path for your other rust projects.

## Rust integration

I hope that this library can eventually be a standard part of the Rust
distribution -- see rust-lang/rust#14031 and rust-lang/rust#6816.  It works
fine as a standalone library, but if it were incorporated, then even Rust's
own libraries could define probe points.

## License

libprobe follows the same license choices as The Rust Project itself, in
order to ease a future merge.

See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.
