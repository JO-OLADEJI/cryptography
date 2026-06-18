use std::collections::BTreeMap;

use crate::{
    exercises::{
        ec_point::{Field, Point},
        finite_field::Fp,
    },
    extension_fields::{
        field_extension::Fp2,
        generator::find_g1s,
        sq_root::{has_root, tonelli_shanks},
        subgroup::embedding_degree,
        utils::{format_curve_equation, gf, prime_factors, MODULUS},
    },
};

/*
 * To brute force curves, the curve and point generation logic needs to be sandwiched in the below snippet
 *
 * ```rust
 * for a in SEARCH_SPACE.0..SEARCH_SPACE.1 {
 *     for b in SEARCH_SPACE.0..SEARCH_SPACE.1 {
 *     if !is_elliptic_curve(a, b) {
 *         continue;
 *     }
 *
 *     // BRUTE FORCE LOGIC
 *
 *     }
 * }
 * ```
 */

pub fn main() {
    let a = -1;
    let b = -1;

    let point_at_infinity = Point::new(gf(a), gf(b), None, None).unwrap();

    let mut points: Vec<Point<Fp>> = vec![];
    points.push(point_at_infinity);

    println!("Curve: {}", format_curve_equation(a, b));
    println!("----------------------------------------------------------");

    for x in 0..MODULUS as i64 {
        // 1a. generate all the points by brute-force and store in a vector
        let value = gf(x).pow(3) + gf(x).scalar_mul_fe(&gf(a)) + gf(b);
        if has_root(&value) {
            let root = tonelli_shanks(&value).unwrap();
            let derived_point = Point::new(gf(a), gf(b), gf(x).into(), gf(root as i64).into());

            // 1b. test the point
            if derived_point.is_ok() {
                // println!("• ({}, {})", x, root);
                points.push(derived_point.unwrap());

                if root != 0 {
                    let root_additive_inverse = MODULUS - root as u32;
                    let derived_point_inverse = Point::new(
                        gf(a),
                        gf(b),
                        gf(x).into(),
                        gf(root_additive_inverse as i64).into(),
                    );
                    // println!("• ({}, {})", x, _root_additive_inverse);
                    points.push(derived_point_inverse.unwrap());
                }
            }
        }
    }
    println!("No. of points #E(Fp): {}", points.len());

    // calculate all prime factors `r` of N(points)
    let potential_subgroups_order = prime_factors(points.len() as u32);
    println!(
        "Potential subgroups order `r` for G1: {:?}",
        potential_subgroups_order
    );

    // for each prime factor, calculate the lowest embedding degree `k` such that:
    // `r | (pᵏ - 1)` (mod p)
    // in other works the smallest `k` such that pᵏ === 1 (mod `r`)
    let mut key: BTreeMap<u32, Option<u32>> = BTreeMap::new();

    for &r in potential_subgroups_order.iter() {
        key.insert(r, embedding_degree(r));
    }
    println!("embedding degrees: {:?}", key);

    let g1s = find_g1s(&points);
    println!("# generators found: {}", g1s.len());

    println!("\n\n\n");
}

pub fn prototype() {
    let x = Fp2::new(-1, 1);
    let y = Fp2::new(5, 3);

    // let sum = x + y;
    let product = x * y;
    let inverse = product.mul_inverse();

    println!("product {}", product);
    println!("inverse {}", inverse);
}
