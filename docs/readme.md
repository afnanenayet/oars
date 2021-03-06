# oars

[![CI](https://github.com/afnanenayet/oars/actions/workflows/CI.yml/badge.svg)](https://github.com/afnanenayet/oars/actions/workflows/CI.yml)
[![crates badge](https://meritbadge.herokuapp.com/oars)](https://crates.io/crates/oars)
[![Documentation](https://docs.rs/oars/badge.svg)](https://docs.rs/oars)
![License](https://img.shields.io/crates/l/oars/0.3.1.svg)

## Summary

oars is a library for constructing orthogonal arrays (OAs) with various
parameters, with the ability to construct OAs using multiple construction
methods. It also provides utilities for constructing strong orthogonal arrays
that work with the facilities for existing OAs or constructing SOAs from
scratch.  On top of being able to generated orthogonal arrays, this crate
provides utilities to verify orthogonal arrays given a set of parameters.

These orthogonal arrays are not predefined and are constructed on the fly. If
you want to create orthogonal arrays using predefined lookup tables, there are
many resources online that provide the numbers for you to use.

For more information about orthogonal arrays and their use in Monte Carlo
sampling, check out [Chapter
10](https://statweb.stanford.edu/~owen/mc/Ch-var-adv.pdf) of Art Owen's Monte
Carlo book.

This library only guarantees stability against stable Rust, unless there is
significant interest in supporting older versions. In such case, please let me
know by either filing an issue on Github or contacting me.

## Development

This project only relies on the
[ndarray](https://github.com/rust-ndarray/ndarray) crate. There are several
optimizations that can be applied to the crate for compilation, such as `BLAS`
acceleration. You can optionally enable `serde` support for the orthogonal
array structs with `features = ["serialize"]`.

This crate was made for the stable compiler, so building is as simple as

```sh
cargo build
# or
cargo build --release
```

You can also run benchmarks using `cargo bench`. This uses criterion to run
benchmarks, which offers robust statistics for benchmarks as well as pretty
output for benchmark results.

### Usage

You can use this library with your Rust project via cargo. It targets the
stable Rust compiler, so as long as you keep Rust updated, you'll probably be
good.

I test this library on Mac/Linux with Travis, and also on my personal Mac and
Linux machines.

Of course, if you find any issues, please file an issue or send a PR. Feature
requests are also welcome.

If you want to enable serialization support for this library, include your
dependency as such:

```toml
[dependencies.oars]
version = "2.1"
features = ["serialize", "parallel"] # optional features
```
