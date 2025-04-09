use byte_repr::represent;

fn main() {
    let x = 68i8;
    represent(&x);

    let x = 42u16;
    represent(&x);

    let x = 16777215u32;
    represent(&x);
}
