use crate::{
    exercises::{ec_point::Point, finite_field::Fp},
    extension_fields::utils::SUBGROUP_ORDER_R,
};

pub fn find_g1s(points: &Vec<Point<Fp>>) -> Vec<&Point<Fp>> {
    let mut g1s: Vec<&Point<Fp>> = vec![];

    for point in points.iter() {
        if !point.is_infinity() && point.scalar_mul(SUBGROUP_ORDER_R).is_infinity() {
            g1s.push(point);
        }
    }

    g1s
}
