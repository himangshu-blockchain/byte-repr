use std::any::type_name;

/// A trait to extract little-endian and big-endian byte representations
/// for any type that supports `to_le_bytes()` and `to_be_bytes()`.
///
/// # Examples
///
/// ```rust
/// use byte_repr::ByteRepr;
///
/// let num: u32 = 0x01020304;
/// assert_eq!(num.to_le_bytes().to_vec(), vec![0x04, 0x03, 0x02, 0x01]);
/// assert_eq!(num.to_be_bytes().to_vec(), vec![0x01, 0x02, 0x03, 0x04]);
/// ```
pub trait ByteRepr {
    /// Returns the little-endian byte representation of the value as a `Vec<u8>`.
    fn to_le_bytes(&self) -> Vec<u8>;

    /// Returns the big-endian byte representation of the value as a `Vec<u8>`.
    fn to_be_bytes(&self) -> Vec<u8>;
}

/// A macro to implement the `ByteRepr` trait for primitive numeric types
/// (e.g., `i8`, `u16`, `u32`, `f64`, etc.).
///
/// It uses the type's built-in methods `to_le_bytes()` and `to_be_bytes()`
/// and converts the resulting fixed-size byte arrays into dynamic vectors.
macro_rules! impl_byterep {
    ($t:ty) => {
        impl ByteRepr for $t {
            fn to_le_bytes(&self) -> Vec<u8> {
                <$t>::to_le_bytes(*self).to_vec()
            }
            fn to_be_bytes(&self) -> Vec<u8> {
                <$t>::to_be_bytes(*self).to_vec()
            }
        }
    };
}

// Implement ByteRepr for selected types
impl_byterep!(i8);
impl_byterep!(u16);
impl_byterep!(u32);

/// A generic function that prints multiple memory-level representations
/// of a numeric value, including binary, little-endian, and big-endian formats.
///
/// It supports:
/// - Binary view
/// - Little-endian byte array and hex
/// - Big-endian byte array and hex
///
/// # Requirements:
/// - The type must implement `Display`, `Binary`, `Copy`, and `ByteRepr`.
///
/// # Color Codes (Terminal):
/// - Binary: Red
/// - LE Bytes: Green
/// - LE Hex: Yellow
/// - BE Bytes: Blue
/// - BE Hex: Magenta
///
/// # Examples
///
/// ```rust
/// use byte_repr::represent;
///
/// let val = 42u16;
/// represent(&val); // Should print representations to stdout
/// ```
pub fn represent<T>(value: &T)
where
    T: std::fmt::Display + std::fmt::Binary + Copy,
    T: ByteRepr,
{
    println!(
        "\n\nHello, Representing in following formats for: {} \x1b[33m({})\x1b[0m",
        value,
        type_name::<T>()
    );

    // Binary
    println!(
        "\x1b[31m Binary representation                      : {:b} \x1b[0m",
        value
    );

    // Little-endian bytes
    println!(
        "\x1b[32m Little-endian byte array                   : {:?} \x1b[0m",
        value.to_le_bytes()
    );
    println!(
        "\x1b[33m Hex memory (LE) with 2-digit zero padding  : {:02x?} \x1b[0m",
        value.to_le_bytes()
    );

    // Big-endian bytes
    println!(
        "\x1b[34m Big-endian byte array                      : {:?} \x1b[0m",
        value.to_be_bytes()
    );
    println!(
        "\x1b[35m Hex memory (BE) with 2-digit zero padding  : {:02x?} \x1b[0m",
        value.to_be_bytes()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_repr_trait_for_i8() {
        let val: i8 = 0x12;
        let expected_le = val.to_le_bytes().to_vec();
        let expected_be = val.to_be_bytes().to_vec();
        assert_eq!(ByteRepr::to_le_bytes(&val), expected_le);
        assert_eq!(ByteRepr::to_be_bytes(&val), expected_be);
    }
    #[test]
    fn test_byte_repr_trait_for_i8_negetive_127() {
        let val: i8 = -127;
        let expected_le = val.to_le_bytes().to_vec();
        let expected_be = val.to_be_bytes().to_vec();
        assert_eq!(ByteRepr::to_le_bytes(&val), expected_le);
        assert_eq!(ByteRepr::to_be_bytes(&val), expected_be);
    }
    #[test]
    fn test_byte_repr_trait_for_u16() {
        let val: u16 = 0x1234;
        let expected_le = val.to_le_bytes().to_vec();
        let expected_be = val.to_be_bytes().to_vec();
        assert_eq!(ByteRepr::to_le_bytes(&val), expected_le);
        assert_eq!(ByteRepr::to_be_bytes(&val), expected_be);
    }

    #[test]
    fn test_byte_repr_trait_for_u32() {
        let val: u32 = 0x123456;
        let expected_le = val.to_le_bytes().to_vec();
        let expected_be = val.to_be_bytes().to_vec();
        assert_eq!(ByteRepr::to_le_bytes(&val), expected_le);
        assert_eq!(ByteRepr::to_be_bytes(&val), expected_be);
    }

    #[test]
    fn test_represent_does_not_panic() {
        let val: i8 = 127;
        represent(&val);
        let val: i8 = -127;
        represent(&val);
        let val: u16 = 65535;
        represent(&val);
        let val: u32 = 4294967295;
        represent(&val);
    }
}
