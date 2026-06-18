use crate::{
    exercises::{ec_point::Point, finite_field::Fp},
    extension_fields::utils::SUBGROUP_ORDER_R,
};

pub fn find_g1s(points: &Vec<Point<Fp>>) -> Vec<&Point<Fp>> {
    let mut g1s: Vec<&Point<Fp>> = vec![];

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
