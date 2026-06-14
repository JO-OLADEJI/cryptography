use std::collections::BTreeMap;

use crate::{
    exercises::ec_point::Point,
    extension_fields::{
        sq_root::{has_root, tonelli_shanks},
        subgroup::embedding_degree,
        utils::{
            format_curve_equation, gf, is_elliptic_curve, prime_factors, MODULUS, SEARCH_SPACE,
        },
    },
};

pub fn main() {
    for a in SEARCH_SPACE.0..SEARCH_SPACE.1 {
        for b in SEARCH_SPACE.0..SEARCH_SPACE.1 {
            if !is_elliptic_curve(a, b) {
                continue;
            }

            let mut no_of_points: u32 = 1; // counting the point at infinity
            println!("Curve: {}", format_curve_equation(a, b));
            println!("----------------------------------------------------------");
            // println!("• (∞, ∞)");

            for x in 0..MODULUS as i64 {
                // 1a. generate all the points by brute-force and store in a vector
                let value = gf(x).pow(3) + gf(x).scalar_mul_fe(&gf(a)) + gf(b);
                if has_root(&value) {
                    let root = tonelli_shanks(&value).unwrap();

                    // 1b. test the point
                    if Point::new(gf(a), gf(b), gf(x).into(), gf(root as i64).into()).is_ok() {
                        // println!("• ({}, {})", x, root);
                        no_of_points += 1;

                        if root != 0 {
                            let _root_additive_inverse = MODULUS - root as u32;
                            // println!("• ({}, {})", x, _root_additive_inverse);
                            no_of_points += 1;
                        }
                    }
                }
            }
            println!("No. of points #E(Fp): {}", no_of_points);

            // 2. calculate all prime factors `r` of N(points)
            let potential_subgroups_order = prime_factors(no_of_points);
            println!(
                "Potential subgroups order `r` for G1: {:?}",
                potential_subgroups_order
            );

            // 3. for each prime factor, calculate the lowest embedding degree `k` such that `r | (p^k - 1)` (mod p)
            // in other works the smallest `k` such that p^k === 1 (mod `r`)
            let mut key: BTreeMap<u32, Option<u32>> = BTreeMap::new();

            for &r in potential_subgroups_order.iter() {
                key.insert(r, embedding_degree(r));
            }
            println!("embedding degrees: {:?}", key);

            println!("\n\n\n");
        }
    }
}
