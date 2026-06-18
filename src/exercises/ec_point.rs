// Book: Programming Bitcoin: Learn How to Program Bitcoin from Scratch by Jimmy song
// Chapter 2, 3: Elliptic Curve Cryptography (over a finite field)

//  ____  __    __    ____  ____  ____  ____   ___        ___  __  __  ____  _  _  ____  ___
// ( ___)(  )  (  )  (_  _)(  _ \(_  _)(_  _) / __)      / __)(  )(  )(  _ \( \/ )( ___)/ __)
// )__)  )(__  )(__  _)(_  )___/  )(   _)(_ ( (__      ( (__  )(__)(  )   / \  /  )__) \__ \
// (____)(____)(____)(____)(__)   (__) (____) \___)      \___)(______)(_)\_)  \/  (____)(___/

use std::fmt::Debug;
use std::fmt::{self, Display};
use std::ops;

/* The general form of the curve is y² = x³ + ax + b, but more specifically the
 * `secp256k1` curve used by bitcoin and ethereum has the equation y² = x³ + 7
 * where `a` equals 0 and `b` equals 7
 *
 * The `Point` struct contains details that satisfy the above equation (general form)
 * over a finite field. `x` and `y` being `None` represents the point at infinity
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<F> {
    pub a: F,
    pub b: F,
    pub x: Option<F>,
    pub y: Option<F>,
}

pub trait Field:
    Copy
    + Clone
    + Debug
    + Display
    + PartialEq
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Neg<Output = Self>
{
    fn mul_inverse(self) -> Self;
    fn pow(self, exp: u32) -> Self;
    fn add_inverse(self) -> Self;
    fn is_zero(self) -> bool;
    fn scalar_mul(self, by: u32) -> Self;
}

impl<F: Field> Point<F> {
    pub fn new(_a: F, _b: F, _x: Option<F>, _y: Option<F>) -> Result<Self, String> {
        match _x {
            Some(x_value) => match _y {
                Some(y_value) => {
                    let lhs = y_value.pow(2);
                    let rhs_0 = x_value.pow(3);
                    let rhs_1 = _a * x_value;
                    let rhs_01 = rhs_0 + rhs_1;
                    let rhs = rhs_01 + _b;

                    if lhs != rhs {
                        return Err(format!(
                            "coordinates ({}, {}) is not on the curve",
                            x_value, y_value
                        ));
                    }

                    Ok(Self {
                        a: _a,
                        b: _b,
                        x: Some(x_value),
                        y: Some(y_value),
                    })
                }
                None => return Err(format!("Invalid infinity point")),
            },
            None => match _y {
                Some(_) => return Err(format!("Invalid infinity point")),
                None => {
                    return Ok(Self {
                        a: _a,
                        b: _b,
                        x: None,
                        y: None,
                    })
                }
            },
        }
    }

    #[allow(dead_code)]
    pub fn scalar_mul(self, by: u32) -> Self {
        let mut product = self;

        if by == 0 {
            return Point::new(self.a, self.b, None, None).unwrap();
        } else if by > 1 {
            // TODO: make this more efficient using "binary expansion"
            for _ in 0..(by - 1) {
                product = (product + self).unwrap();
            }
        }

        product
    }

    #[allow(dead_code)]
    pub fn is_infinity(self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}

impl<F: Field> ops::Add for Point<F> {
    type Output = Result<Self, String>;

    fn add(self, point_2: Self) -> Self::Output {
        if self.a != point_2.a || self.b != point_2.b {
            return Err(format!("Point addition invalid on different curves"));
        }

        let slope: F;

        /*
         * Case 1(a): first point is at infinity P₁ = P(∞)
         */
        if self.x.is_none() && self.y.is_none() {
            if point_2.x.is_none() && point_2.y.is_none() {
                return Ok(Point::new(self.a, self.b, None, None).unwrap());
            }

            return Ok(Point::new(
                self.a,
                self.b,
                point_2.x,
                point_2.y.map(|y| y.add_inverse()),
            )
            .unwrap());
        }
        /*
         * Case 1(b): second point is at infinity P₂ = P(∞)
         */
        else if point_2.x.is_none() && point_2.y.is_none() {
            if self.x.is_none() && self.y.is_none() {
                return Ok(Point::new(self.a, self.b, None, None).unwrap());
            }

            // if the other point is on the curve, we flip the `y` value
            return Ok(
                Point::new(self.a, self.b, self.x, self.y.map(|y| y.add_inverse())).unwrap(),
            );
        }

        let x1_value = self.x.unwrap();
        let y1_value = self.y.unwrap();
        let x2_value = point_2.x.unwrap();
        let y2_value = point_2.y.unwrap();

        /*
         * Case 2: same points where P₁ == P₂
         * Same formula as Case 3, except:
         *
         * s (slope) = (3x² + a)/2y;  | dy/dx => y² = x³ + ax + b
         */
        if self.x == point_2.x && self.y == point_2.y {
            /*
             * Case 2 (variant): same points where P₁ == P₂ and `y` = 0; `s` denominator results in zero
             * meaning slope is `undefined`. This results in P(∞)
             */
            if self.y.unwrap().is_zero() {
                return Ok(Point::new(self.a, self.b, None, None).unwrap());
            }

            slope = (x1_value.pow(2).scalar_mul(3) + self.a) / y1_value.scalar_mul(2);
        }
        /*
         * Case 3 (base case): distinct points where P₁ != P₂
         * P₁ = (x₁, y₁);  P₂ = (x₂, y₂);  P₃ = (x₃, y₃)
         * P₁ + P₂ = P₃
         *
         * s (slope) = (y₂ - y₁) / (x₂ - x₁)
         * x₃ = s² - x₁ - x₂
         * y₃ = s(x₁ - x₃) - y₁
         */
        else {
            /*
             * Case 3 (variant) - if the two `x` points are equivalent and `y` points are negated, i.e point_a.x == point_b.x && point_a.y == -(point_b.y)
             * This results in the infinity point
             */
            if self.x == point_2.x && (y1_value + y2_value).is_zero() {
                return Ok(Point::new(self.a, self.b, None, None).unwrap());
            }

            slope = (y2_value - y1_value) / (x2_value - x1_value);
        }

        let point_3_x = (slope.pow(2) - x1_value) - x2_value;
        let point_3_y = (slope * (x1_value - point_3_x)) - y1_value;

        Ok(Point::new(self.a, self.b, Some(point_3_x), Some(point_3_y)).unwrap())
    }
}

impl<F: Field> fmt::Display for Point<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.x.is_none() && self.y.is_none() {
            return write!(f, "• (∞, ∞)");
        }

        write!(f, "• ({}, {})", self.x.unwrap(), self.y.unwrap(),)
    }
}

#[cfg(test)]
mod ecc_tests {
    use crate::exercises::finite_field::Fp;

    use super::*;

    const ORDER: u32 = 7;
    const ORDER_2: u32 = 11;

    const SECP256K1_A: Fp = Fp {
        num: 0,
        modulus: ORDER,
    };
    const SECP256K1_B: Fp = Fp {
        num: 7 % ORDER,
        modulus: ORDER,
    };

    #[test]
    fn test_point_init_error_infinity() {
        let x = Fp::new(0, ORDER_2).unwrap();
        let y = Fp::new(0, ORDER_2).unwrap();

        assert_eq!(
            Point::new(SECP256K1_A, SECP256K1_B, Some(x), None),
            Err(format!("Invalid infinity point"))
        );
        assert_eq!(
            Point::new(SECP256K1_A, SECP256K1_B, None, Some(y)),
            Err(format!("Invalid infinity point"))
        );
    }

    #[test]
    fn test_point_init_error() {
        let x = Fp::new(0, ORDER).unwrap();
        let y = Fp::new(1, ORDER).unwrap();

        assert_eq!(
            Point::new(SECP256K1_A, SECP256K1_B, Some(x), Some(y)),
            Err(format!("coordinates ({}, {}) is not on the curve", x, y))
        );
    }

    #[test]
    fn test_point_init() {
        let x = Fp::new(0, ORDER).unwrap();
        let y = Fp::new(0, ORDER).unwrap();

        assert_eq!(
            Point::new(SECP256K1_A, SECP256K1_B, Some(x), Some(y)),
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: Some(x),
                y: Some(y)
            })
        );
    }

    #[test]
    fn test_point_init_infinity() {
        let x = None;
        let y = None;

        assert_eq!(
            Point::new(SECP256K1_A, SECP256K1_B, x, y),
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: None,
                y: None
            })
        );
    }

    #[test]
    fn test_point_addition_distinct() {
        let x1 = Fp::new(1, ORDER).unwrap();
        let y1 = Fp::new(6, ORDER).unwrap();
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, Some(x1), Some(y1)).unwrap();

        let x2 = Fp::new(2, ORDER).unwrap();
        let y2 = Fp::new(1, ORDER).unwrap();
        let point_b = Point::new(SECP256K1_A, SECP256K1_B, Some(x2), Some(y2)).unwrap();

        assert_eq!(
            point_a + point_b,
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: Some(Fp::new(1, ORDER).unwrap()),
                y: Some(Fp::new(1, ORDER).unwrap()),
            })
        )
    }

    #[test]
    fn test_point_addition_equal() {
        let x = Fp::new(1, ORDER).unwrap();
        let y = Fp::new(6, ORDER).unwrap();
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, Some(x), Some(y)).unwrap();

        println!("{:?}", point_a + point_a);

        assert_eq!(
            point_a + point_a,
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: Some(Fp::new(2, ORDER).unwrap()),
                y: Some(Fp::new(6, ORDER).unwrap()),
            })
        )
    }

    #[test]
    fn test_point_addition_equal_vertical_line() {
        let x = Fp::new(0, ORDER).unwrap();
        let y = Fp::new(0, ORDER).unwrap();
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, Some(x), Some(y)).unwrap();

        assert_eq!(
            point_a + point_a,
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: None,
                y: None,
            })
        )
    }

    #[test]
    fn test_point_addition_infinity() {
        let x = None;
        let y = None;
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, x, y).unwrap();

        assert_eq!(
            point_a + point_a,
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: None,
                y: None,
            })
        )
    }

    #[test]
    fn test_point_addition_identity() {
        let x = None;
        let y = None;
        let point_infinity = Point::new(SECP256K1_A, SECP256K1_B, x, y).unwrap();

        let x1 = Fp::new(1, ORDER).unwrap();
        let y1 = Fp::new(6, ORDER).unwrap();
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, Some(x1), Some(y1)).unwrap();

        let x2 = Fp::new(4, ORDER).unwrap();
        let y2 = Fp::new(1, ORDER).unwrap();
        let point_b = Point::new(SECP256K1_A, SECP256K1_B, Some(x2), Some(y2)).unwrap();

        assert_eq!(
            point_infinity + point_a,
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: Some(Fp::new(1, ORDER).unwrap()),
                y: Some(Fp::new(1, ORDER).unwrap()),
            })
        );

        assert_eq!(
            point_b + point_infinity,
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: Some(Fp::new(4, ORDER).unwrap()),
                y: Some(Fp::new(6, ORDER).unwrap()),
            })
        );
    }

    #[test]
    fn test_point_addition_identity_variation() {
        let x1 = Fp::new(1, ORDER).unwrap();
        let y1 = Fp::new(6, ORDER).unwrap();
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, Some(x1), Some(y1)).unwrap();

        let x2 = Fp::new(1, ORDER).unwrap();
        let y2 = Fp::new(1, ORDER).unwrap();
        let point_b = Point::new(SECP256K1_A, SECP256K1_B, Some(x2), Some(y2)).unwrap();

        assert_eq!(
            point_a + point_b,
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: None,
                y: None,
            })
        );
    }

    #[test]
    fn test_scalar_multiplication() {
        let x = Fp::new(1, ORDER).unwrap();
        let y = Fp::new(6, ORDER).unwrap();
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, Some(x), Some(y)).unwrap();

        assert_eq!(
            point_a.scalar_mul(5),
            Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: Some(Fp::new(2, ORDER).unwrap()),
                y: Some(Fp::new(1, ORDER).unwrap()),
            }
        );
    }
}
