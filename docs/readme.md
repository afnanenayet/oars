# oa-rs

[![Build Status](https://travis-ci.com/afnanenayet/oars.svg?token=QtxzrX3Qc2BDQfwx8D1K&branch=master)](https://travis-ci.com/afnanenayet/oars)
[![crates badge](https://meritbadge.herokuapp.com/oars)](https://crates.io/crates/oars)
[![License](https://img.shields.io/crates/l/oars/0.3.1.svg)]

## Summary

oa-rs/oars is a library for constructing orthogonal arrays (OAs) with various
parameters, with the ability to construct OAs using multiple construction
methods. It also provides utilities for constructing strong orthogonal arrays
that work with the facilities for existing OAs or constructing SOAs from
scratch.  On top of being able to generated orthogonal arrays, this crate
provides utilities to verify orthogonal arrays given a set of parameters.

These orthogonal arrays are not predefined and are constructed on the fly.  If
you want to create orthogonal arrays using predefined lookup tables, there are
many resources online that provide the numbers for you to use.

For more information about orthogonal arrays and their use in Monte Carlo
sampling, check out [Chapter
10](https://statweb.stanford.edu/~owen/mc/Ch-var-adv.pdf) of Art Owen's Monte
Carlo book.

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

This will also be available at https://crates.io.

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
version = "0.3"
features = ["serialize"]
```

## Roadmap

- [ ] provide parallelized constructors and method variants
