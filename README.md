# byte-repr

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

```
Hello, Representing in following formats for: 42 (u16)
 Binary representation                      : 101010 
 Little-endian byte array                   : [42, 0] 
 Hex memory (LE) with 2-digit zero padding  : [2a, 00] 
 Big-endian byte array                      : [0, 42] 
 Hex memory (BE) with 2-digit zero padding  : [00, 2a] 
```

## 🔧 Supported Types

Currently supports:
- `i8`
- `u16`
- `u32`

More can be added using the `impl_byterep!` macro.

## 📦 License

Licensed under MIT OR Apache-2.0.
