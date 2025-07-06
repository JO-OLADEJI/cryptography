use crate::exercises::{ch1::FieldElement, ch2::Point};

mod exercises;

const ORDER: u32 = 223;

fn main() {
    let secp256k1_a = FieldElement::new(0, ORDER).unwrap();
    let secp256k1_b = FieldElement::new(7, ORDER).unwrap();

    // • (47, 71)
    let point_a = Point::new(
        secp256k1_a,
        secp256k1_b,
        Some(FieldElement::new(47, ORDER).unwrap()),
        Some(FieldElement::new(71, ORDER).unwrap()),
    );

    // • (47, 152)
    // let point_b = Point::new(
    //     secp256k1_a,
    //     secp256k1_b,
    //     Some(FieldElement::new(47, ORDER).unwrap()),
    //     Some(FieldElement::new(152, ORDER).unwrap()),
    // );

    for i in 1..=21 {
        println!("{} => {}", i, point_a.as_ref().unwrap().scalar_mul(i));
    }

    // println!("Programming Bitcoin!");
}
