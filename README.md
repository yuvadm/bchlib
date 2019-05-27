# bchlib

![Crates.io](https://img.shields.io/crates/v/bchlib.svg)

Rust bindings for BCH encoding/decoding library, based on the [bch_codec](https://github.com/mborgerding/bch_codec) fork.

This workspace repo contains both the high-level `bchlib` project as well as the lower-level `bchlib-sys` project that builds the original C library.

## Usage

Add the library to your `Cargo.toml`:

```
[dependencies]
bchlib = "0.1.0"
```

## Build

The usual:

```bash
$ cargo build
$ cargo test
```

Note that due to usage of `bindgen` in the lower level `bchlib-sys` project, you will need `clang` to be installed on your system.

## License

[GPLv2](LICENSE.md)
