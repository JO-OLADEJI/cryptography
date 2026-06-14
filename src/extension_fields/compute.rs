use crate::{
    exercises::ec_point::Point,
    extension_fields::{
        sq_root::{has_root, tonelli_shanks},
        utils::{format_curve_equation, gf, is_elliptic_curve, MODULUS},
    },
};

pub fn main() {
    for a in 0..3 {
        for b in 0..3 {
            if !is_elliptic_curve(a, b) {
                continue;
            }

            let mut no_of_points = 1; // counting the point at infinity
            println!("Curve: {}", format_curve_equation(a, b));
            println!("----------------------------------------------------------");
            println!("• (∞, ∞)");

            for x in 0..MODULUS as i64 {
                // 1a. generate all the points by brute-force and store in a vector
                let value = gf(x).pow(3) + gf(x).scalar_mul_fe(&gf(a)) + gf(b);
                if has_root(&value) {
                    let root = tonelli_shanks(&value).unwrap();

                    // 1b. test the point
                    if Point::new(gf(a), gf(b), gf(x).into(), gf(root as i64).into()).is_ok() {
                        println!("• ({}, {})", x, root);
                        no_of_points += 1;

                        if root != 0 {
                            let root_additive_inverse = MODULUS - root as u32;
                            println!("• ({}, {})", x, root_additive_inverse);
                            no_of_points += 1;
                        }
                    }
                }
            }

            println!("No. of points #E(Fp): {}", no_of_points);
            println!("\n\n\n");
        }
    }

    // 2. calculate all prime factors `r` of N(points)
    // 3. for each prime factor, calculate the lowest embedding degree `k` such that `r | (p^k - 1)` (mod p)
}

// 2	3	5	7	11	13	17	19	23	29	31	37	41	43	47	53	59	61	67	71
// 73	79	83	89	97	101	103	107	109	113	127	131	137	139	149	151	157	163	167	173
// 179	181	191	193	197	199	211	223	227	229	233	239	241	251	257	263	269	271	277	281
// 283	293	307	311	313	317	331	337	347	349	353	359	367	373	379	383	389	397	401	409
// 419	421	431	433	439	443	449	457	461	463	467	479	487	491	499	503	509	521	523	541
