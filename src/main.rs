use crate::exercises::{ch1::FieldElement, ch2::Point};

mod exercises;

const MODULUS: u32 = 43;

fn main() {
    let secp256k1_a = FieldElement::new(0, MODULUS).unwrap();
    let secp256k1_b = FieldElement::new(7, MODULUS).unwrap();

    // • (42, 7)
    let point_a = Point::new(
        secp256k1_a,
        secp256k1_b,
        Some(FieldElement::new(42, MODULUS).unwrap()),
        Some(FieldElement::new(7, MODULUS).unwrap()),
    )
    .unwrap();

    for i in 1..=57 {
        let point: Point = point_a.scalar_mul(i);
        println!("{} => {}", i, &point);

        if point.x.is_none() && point.y.is_none() {
            break;
        }
    }
}
