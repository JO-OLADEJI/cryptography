mod exercises;
mod extension_fields;

fn main() {
    // let a = FieldElement::new(2, MODULUS).unwrap();
    // let b = FieldElement::new(3, MODULUS).unwrap();

    extension_fields::compute::prototype();

    // • (3, 6)
    // let potential_g = Point::new(
    //     a,
    //     b,
    //     Some(FieldElement::new(3, MODULUS).unwrap()),
    //     Some(FieldElement::new(6, MODULUS).unwrap()),
    // )
    // .unwrap();

    // for i in 1..=MODULUS {
    //     let point: Point = potential_g.scalar_mul(i);
    //     println!("{} => {}", i, &point);

    //     if point.x.is_none() && point.y.is_none() {
    //         break;
    //     }
    // }
}
