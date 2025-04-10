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
/// (e.g., `u8`, `u16`, `u32`, `f64`, etc.).
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

// Implement ByteRepr for all unsigned integer types
impl_byterep!(u8);
impl_byterep!(u16);
impl_byterep!(u32);
impl_byterep!(u64);
impl_byterep!(u128);
impl_byterep!(usize);

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
///     let x = 255u8;
///     represent(&x);
///
///     let x = 65535u16;
///     represent(&x);
///
///     let x = 4294967295u32;
///     represent(&x);
///
///     let x = 18446744073709551615u64;
///     represent(&x);
///
///     let x = 340282366920938463463374607431768211455u128;
///     represent(&x);
///
///     let x = 18446744073709551615usize;
///     represent(&x);
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

    macro_rules! test_unsigned {
        ($name:ident, $ty:ty, $val:expr) => {
            #[test]
            fn $name() {
                let val: $ty = $val;
                let expected_le = val.to_le_bytes().to_vec();
                let expected_be = val.to_be_bytes().to_vec();
                assert_eq!(ByteRepr::to_le_bytes(&val), expected_le);
                assert_eq!(ByteRepr::to_be_bytes(&val), expected_be);
            }
        };
    }

    test_unsigned!(test_u8, u8, 0x12);
    test_unsigned!(test_u16, u16, 0x1234);
    test_unsigned!(test_u32, u32, 0x12345678);
    test_unsigned!(test_u64, u64, 0x1234567890abcdef);
    test_unsigned!(test_u128, u128, 0x1234567890abcdef1122334455667788);
    test_unsigned!(test_usize, usize, 0x1234);

    #[test]
    fn test_represent_does_not_panic() {
        represent(&42u8);
        represent(&42u16);
        represent(&42u32);
        represent(&42u64);
        represent(&42u128);
        represent(&42usize);
    }
}
