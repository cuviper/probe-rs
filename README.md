# libprobe: Static probes for Rust

[![probe crate](https://img.shields.io/crates/v/probe.svg)](https://crates.io/crates/probe)
![minimum rustc 1.66](https://img.shields.io/badge/rustc-1.66+-red.svg)
[![probe documentation](https://docs.rs/probe/badge.svg)](https://docs.rs/probe)
[![build status](https://github.com/cuviper/rust-libprobe/workflows/CI/badge.svg)](https://github.com/cuviper/rust-libprobe/actions)

With the `probe!` macro, programmers can place static instrumentation
points in their code to mark events of interest. These are compiled into
platform-specific implementations, e.g. SystemTap SDT on Linux. Probes are
designed to have negligible overhead during normal operation, so they can
be present in all builds, and only activated using those external tools.

[Documentation](https://docs.rs/probe/)

## Using libprobe

[`probe!` is available on crates.io](https://crates.io/crates/probe).
The recommended way to use it is to add a line into your Cargo.toml such as:

```toml
[dependencies]
probe = "0.5"
```

Then `use probe::probe;` in your code and insert macro calls wherever you want
to mark something, `probe!(provider, name, args...)`. The `provider` and `name`
are identifiers of your choice, and any additional arguments are runtime
expressions that will be cast `as isize` for the probe consumer to read.
There is also a `probe_lazy!` variant that tries to avoid evaluating the
argument expressions when probes aren't in use, if the platform-specific
implementation allows that to be determined.

## Rust integration

I hope that this library can eventually be a standard part of the Rust
distribution -- see the rust [pull request][libprobe-pr] and [original
enhancement issue][dtrace-issue]. It works fine as a standalone library,
but if it were incorporated, then even Rust's own libraries could define
probe points.

[libprobe-pr]: https://github.com/rust-lang/rust/pull/14031
[dtrace-issue]: https://github.com/rust-lang/rust/issues/6816

## License

libprobe follows the same license choices as The Rust Project itself, in
order to ease a future merge.

See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.
