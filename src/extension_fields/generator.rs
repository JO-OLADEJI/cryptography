use crate::{
    exercises::ec_point::Point,
    extension_fields::utils::{gf, MODULUS, SUBGROUP_ORDER_R},
};

pub fn find_g1s(points: &Vec<Point>) -> Vec<&Point> {
    let mut g1s: Vec<&Point> = vec![];

    for point in points.iter() {
        let mut order: usize = 1;
        let mut cursor = point.clone();

        while !cursor.is_infinity() {
            cursor = (cursor + *point).unwrap();
            order += 1;

            if cursor.is_infinity() && order as u32 == SUBGROUP_ORDER_R {
                g1s.push(point);
                break;
            }
        }
    }

    g1s
}

pub fn get_non_residue() -> Option<u32> {
    for n in 2..MODULUS {
        let x = gf(n as i64);

        if x.pow(SUBGROUP_ORDER_R - 1).num == MODULUS - 1 {
            return Some(n);
        }
    }

    None
}

// pub fn get_non_residue() -> Option<u32> {
//     for n in 2..157 {
//         let x = gf(n as i64);
//         if x.pow(78).num == 156 {
//             // println!("non-residue = {}", n);
//             return Some(n);
//         }
//     }

//     None
// }
