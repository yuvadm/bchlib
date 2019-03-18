# bchlib-sys

![Crates.io](https://img.shields.io/crates/v/bchlib-sys.svg)

Low-level Rust bindings for BCH encoding/decoding library, based on the [bch_codec](https://github.com/mborgerding/bch_codec) fork.

The higher level library can be found at: https://github.com/yuvadm/bchlib

## Usage

Add the library to your `Cargo.toml`:

```
[dependencies]
bchlib-sys = "0.1.0"
```

## Build

The usual:

```bash
$ cargo build
$ cargo test
```

## License

[GPLv2](LICENSE.md)
