// tests/byte_repr_test.rs

use byte_repr::{ByteRepr, represent};

// ByteRepr trait tests

#[test]
fn test_byte_repr_for_u8() {
    let val: u8 = 0x12;
    assert_eq!(val.to_le_bytes().to_vec(), ByteRepr::to_le_bytes(&val));
    assert_eq!(val.to_be_bytes().to_vec(), ByteRepr::to_be_bytes(&val));
}

#[test]
fn test_byte_repr_for_u16() {
    let val: u16 = 0x1234;
    assert_eq!(val.to_le_bytes().to_vec(), ByteRepr::to_le_bytes(&val));
    assert_eq!(val.to_be_bytes().to_vec(), ByteRepr::to_be_bytes(&val));
}

#[test]
fn test_byte_repr_for_u32() {
    let val: u32 = 0x12345678;
    assert_eq!(val.to_le_bytes().to_vec(), ByteRepr::to_le_bytes(&val));
    assert_eq!(val.to_be_bytes().to_vec(), ByteRepr::to_be_bytes(&val));
}

#[test]
fn test_byte_repr_for_u64() {
    let val: u64 = 0x1234567890ABCDEF;
    assert_eq!(val.to_le_bytes().to_vec(), ByteRepr::to_le_bytes(&val));
    assert_eq!(val.to_be_bytes().to_vec(), ByteRepr::to_be_bytes(&val));
}

#[test]
fn test_byte_repr_for_u128() {
    let val: u128 = 0x1234567890ABCDEF1234567890ABCDEF;
    assert_eq!(val.to_le_bytes().to_vec(), ByteRepr::to_le_bytes(&val));
    assert_eq!(val.to_be_bytes().to_vec(), ByteRepr::to_be_bytes(&val));
}

#[test]
fn test_byte_repr_for_usize() {
    let val: usize = 0xDEADBEEF;
    assert_eq!(val.to_le_bytes().to_vec(), ByteRepr::to_le_bytes(&val));
    assert_eq!(val.to_be_bytes().to_vec(), ByteRepr::to_be_bytes(&val));
}

// represent() tests

#[test]
fn test_represent_for_u8() {
    let val: u8 = 0x7F;
    represent(&val);
}

#[test]
fn test_represent_for_u16() {
    let val: u16 = 0xBEEF;
    represent(&val);
}

#[test]
fn test_represent_for_u32() {
    let val: u32 = 0xDEADBEEF;
    represent(&val);
}

#[test]
fn test_represent_for_u64() {
    let val: u64 = 0x123456789ABCDEF0;
    represent(&val);
}

#[test]
fn test_represent_for_u128() {
    let val: u128 = 0x1234567890ABCDEF1234567890ABCDEF;
    represent(&val);
}

#[test]
fn test_represent_for_usize() {
    let val: usize = 0xDEADCAFE;
    represent(&val);
}
