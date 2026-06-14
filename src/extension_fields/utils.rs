use std::u32;

use crate::exercises::finite_field::FieldElement;

pub const MODULUS: u32 = 11;

pub fn gf(value: i64) -> FieldElement {
    FieldElement::new(value, MODULUS).unwrap()
}

pub fn format_curve_equation(a: i64, b: i64) -> String {
    let mut result = String::from("y² = x³");

    match a {
        i64::MIN..0 => result.push_str(&format!(" - {}x", a.abs())), // there's a subtle bug on this line: i64::MIN.abs() will panic
        0 => {}
        1..=i64::MAX => result.push_str(&format!(" + {}x", a)),
    };

    match b {
        i64::MIN..0 => result.push_str(&format!(" - {}", b.abs())), // ditto
        0 => {}
        1..=i64::MAX => result.push_str(&format!(" + {}", b)),
    };

    result
}

pub fn is_elliptic_curve(a: i64, b: i64) -> bool {
    let discriminant = gf(4) * gf(a).pow(3) + gf(27) * gf(b).pow(2);

    discriminant != gf(0)
}
