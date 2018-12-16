# oa-rs

## Summary

oa-rs/oars is a library for constructing orthogonal arrays (OAs) with various
parameters, with the ability to construct OAs using multiple construction
methods.

These orthogonal arrays are not predefined and are constructed on the fly.
If you want to create orthogonal arrays using predefined lookup tables,
there are many resources online that provide the numbers for you to use.

## Development

This project only relies on the [ndarray](https://github.com/rust-ndarray/ndarray)
crate. There are several optimizations that can be applied to the crate for
compilation, such as `BLAS` acceleration.

This crate was made for the stable compiler, so building is as simple as

```sh
cargo build 
# or
cargo build --release
```

This will also be available in crates.io

### Usage

You can use this library with your Rust project via cargo. It targets the
stable Rust compiler, so as long as you keep Rust updated, you'll probably
be good.

I test this library on Mac/Linux with Travis, and also on my personal
Mac and Linux machines.

Of course, if you find any issues, please feel free to file an issue or
send a PR. Feature requests are also welcome.

## Roadmap

- [x] implement Bose constructor
- [ ] implement Bush constructor
- [ ] provide parallelized constructors and method variants
