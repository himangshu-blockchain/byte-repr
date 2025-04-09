// tests/byte_repr_test.rs

use byte_repr::{ByteRepr, represent};

#[test]
fn test_byte_repr_for_u32() {
    let val: u32 = 0x01020304;
    let expected_le = vec![0x04, 0x03, 0x02, 0x01];
    let expected_be = vec![0x01, 0x02, 0x03, 0x04];

    assert_eq!(val.to_le_bytes().to_vec(), expected_le);
    assert_eq!(val.to_be_bytes().to_vec(), expected_be);
    assert_eq!(ByteRepr::to_le_bytes(&val), expected_le);
    assert_eq!(ByteRepr::to_be_bytes(&val), expected_be);
}

#[test]
fn test_represent_executes() {
    let val = 255u32;
    represent(&val); // Just checking it runs without panic
}
