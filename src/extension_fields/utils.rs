use std::u32;

use crate::exercises::finite_field::FieldElement;

pub const MODULUS: u32 = 157;

pub const SEARCH_SPACE: (i64, i64) = (-10, 10);

const FIRST_100_PRIMES: [u32; 100] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
];

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

// NOTE: factorization uses only the primes in FIRST_100_PRIMES.
// Results are only guaranteed correct when all prime factors
// are <= 541 or the remaining cofactor is itself prime.
pub fn prime_factors(n: u32) -> Vec<u32> {
    let mut n_mut = n;
    let mut factors = Vec::new();

    for &prime in FIRST_100_PRIMES.iter() {
        if prime * prime > n_mut {
            break;
        }

        if n_mut % prime == 0 {
            factors.push(prime);

            while n_mut % prime == 0 {
                n_mut /= prime;
            }
        }
    }

    if n_mut > 1 {
        factors.push(n_mut);
    }

    factors
}
