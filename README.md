# libprobe: Static probes for Rust

With the `probe!` macro, programmers can place static instrumentation
points in their code to mark events of interest.  These are compiled into
platform-specific implementations, e.g. SystemTap SDT on Linux.  Probes are
designed to have negligible overhead during normal operation, so they can
be present in all builds, and only activated using those external tools.

## Documentation

Generated documentation for libprobe can be found
[here](https://cuviper.github.io/rust-libprobe/doc/probe/index.html).

## Building libprobe

Simply run `rustc src/lib.rs` and copy the libprobe shared object to an
appropriate library path for your other rust projects.

If you're using cargo, just add this to your `Cargo.toml`:

```toml
[dependencies.probe]
git = "https://github.com/cuviper/rust-libprobe.git"
```

## Rust integration

I hope that this library can eventually be a standard part of the Rust
distribution -- see the rust [pull request][libprobe-pr] and [original
enhancement issue][dtrace-issue].  It works fine as a standalone library,
but if it were incorporated, then even Rust's own libraries could define
probe points.

[libprobe-pr]: https://github.com/rust-lang/rust/pull/14031
[dtrace-issue]: https://github.com/rust-lang/rust/issues/6816

## License

libprobe follows the same license choices as The Rust Project itself, in
order to ease a future merge.

See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.
