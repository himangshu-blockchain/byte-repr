# 🎓 Rust Crate Experiment | byte-repr

[![Crates.io](https://img.shields.io/crates/v/byte-repr.svg)](https://crates.io/crates/byte-repr)
[![Docs.rs](https://docs.rs/byte-repr/badge.svg)](https://docs.rs/byte-repr)
[![Build Status](https://github.com/himangshu-blockchain/byte-repr/workflows/CI/badge.svg)](https://github.com/himangshu-blockchain/byte-repr/actions)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/byte-repr)](https://github.com/himangshu-blockchain/byte-repr/blob/main/LICENSE)
[![Downloads](https://img.shields.io/crates/d/byte-repr.svg)](https://crates.io/crates/byte-repr)
[![MSRV](https://img.shields.io/badge/MSRV-1.60+-blue.svg)](https://docs.rs/byte-repr)  <!-- Change 1.60+ to your actual MSRV -->

`byte-repr` is a lightweight Rust crate for inspecting the memory representation of numeric values. It helps visualize the binary, little-endian, and big-endian byte representations, including their hex format with zero padding.

Useful for:
- Learning and teaching byte order (endianness)
- Debugging binary data
- Understanding how integers are stored in memory

## ✨ Features

- Print values in:
  - Binary format
  - Little-endian byte array
  - Big-endian byte array
  - Hexadecimal representation with zero padding
- Color-coded output for clarity
- Macro-based extensibility for numeric types

## 🚀 Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
byte-repr = "0.1.0" # update after publishing
```

Then use in your code:

```rust
use byte_repr::represent;

fn main() {
    let x = 42u16;
    represent(&x);
}
```

### Example Output:

![example output](docs/example_output.png)

## 🔧 Supported Types

Currently supports:
- `i8`
- `u16`
- `u32`

More can be added using the `impl_byterep!` macro.

## 📦 License

Licensed under MIT OR Apache-2.0.
