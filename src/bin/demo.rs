use byte_repr::represent;

fn main() {
    let x = 255u8;
    represent(&x);

    let x = 65535u16;
    represent(&x);

    let x = 4294967295u32;
    represent(&x);

    let x = 18446744073709551615u64;
    represent(&x);

    let x = 340282366920938463463374607431768211455u128;
    represent(&x);

    let x = 18446744073709551615usize;
    represent(&x);
}
