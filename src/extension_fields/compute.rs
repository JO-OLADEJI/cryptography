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
        utils::{format_curve_equation, gf, prime_factors, MODULUS, SUBGROUP_ORDER_R},
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
    println!("{}", g1s[12]);
    println!("# generators found: {}", g1s.len());

    println!("\n\n\n");
}

pub fn prototype() {
    let mut field_elements: Vec<Fp2> = vec![];

    for a in 0..MODULUS {
        for b in 0..MODULUS {
            let arbitrary_point = Fp2::new(a.into(), b.into());
            field_elements.push(arbitrary_point);
        }
    }

    for x in field_elements[5000..10000].iter() {
        for y in field_elements.iter() {
            let point = Point::new(Fp2::new(-1, 0), Fp2::new(-1, 0), Some(*x), Some(*y));
            if point.is_ok()
                && !point.clone().unwrap().is_infinity()
                && point
                    .clone()
                    .unwrap()
                    .scalar_mul(SUBGROUP_ORDER_R)
                    .is_infinity()
            {
                println!("G2 {}", point.unwrap());
                return;
            }
        }
    }
}
