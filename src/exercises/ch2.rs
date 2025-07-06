// Book: Programming Bitcoin: Learn How to Program Bitcoin from Scratch by Jimmy song
// Chapter 2, 3: Elliptic Curve Cryptography (over a finite field)

//  ____  __    __    ____  ____  ____  ____   ___        ___  __  __  ____  _  _  ____  ___
// ( ___)(  )  (  )  (_  _)(  _ \(_  _)(_  _) / __)      / __)(  )(  )(  _ \( \/ )( ___)/ __)
// )__)  )(__  )(__  _)(_  )___/  )(   _)(_ ( (__      ( (__  )(__)(  )   / \  /  )__) \__ \
// (____)(____)(____)(____)(__)   (__) (____) \___)      \___)(______)(_)\_)  \/  (____)(___/

use crate::exercises::ch1::FieldElement;
use std::fmt;
use std::ops;

/* The general form of the curve is y² = x³ + ax + b, but more specifically the
 * `secp256k1` curve used by bitcoin and ethereum has the equation y² = x³ + 7
 * where `a` equals 0 and `b` equals 7
 *
 * The `Point` struct contains details that satisfy the above equation (general form)
 * over a finite field. `x` and `y` being `None` represents the point at infinity
 */
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    a: FieldElement,
    b: FieldElement,
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl Point {
    pub fn new(
        _a: FieldElement,
        _b: FieldElement,
        _x: Option<FieldElement>,
        _y: Option<FieldElement>,
    ) -> Result<Self, String> {
        match _x {
            Some(x_value) => match _y {
                Some(y_value) => {
                    if _a.prime != _b.prime
                        || _b.prime != x_value.prime
                        || x_value.prime != y_value.prime
                    {
                        return Err(format!("Cannot operate on different Fields"));
                    }

                    let lhs = y_value.pow(2);
                    let rhs_0 = x_value.pow(3);
                    let rhs_1 = (_a * x_value).unwrap();
                    let rhs_01 = (rhs_0 + rhs_1).unwrap();
                    let rhs = (rhs_01 + _b).unwrap();

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

    pub fn scalar_mul(self, by: u32) -> Self {
        // let mut product =
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
}

impl ops::Add for Point {
    type Output = Result<Self, String>;

    fn add(self, point_2: Self) -> Self::Output {
        if self.a != point_2.a || self.b != point_2.b {
            return Err(format!("Point addition invalid on different curves"));
        }

        let slope: FieldElement;

        /*
         * Case 1(a): first point is at infinity P₁ = P(∞)
         */
        if self.x == None && self.y == None {
            if point_2.x == None && point_2.y == None {
                return Ok(Point::new(self.a, self.b, None, None).unwrap());
            }

            return Ok(Point::new(
                self.a,
                self.b,
                point_2.x,
                Some(
                    FieldElement::new(
                        self.a.prime - point_2.y.unwrap().num, // flip `y` on x-axis
                        self.a.prime,
                    )
                    .unwrap(),
                ),
            )
            .unwrap());
        }
        /*
         * Case 1(b): second point is at infinity P₂ = P(∞)
         */
        else if point_2.x == None && point_2.y == None {
            if self.x == None && self.y == None {
                return Ok(Point::new(self.a, self.b, None, None).unwrap());
            }

            // if the other point is on the curve, we flip the `y` value
            return Ok(Point::new(
                self.a,
                self.b,
                self.x,
                Some(
                    FieldElement::new(
                        self.a.prime - self.y.unwrap().num, // flip `y` on x-axis
                        self.a.prime,
                    )
                    .unwrap(),
                ),
            )
            .unwrap());
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
            if self.y.unwrap().num == 0 {
                return Ok(Point::new(self.a, self.b, None, None).unwrap());
            }

            slope = ((x1_value.pow(2).scalar_mul(3) + self.a).unwrap() / y1_value.scalar_mul(2))
                .unwrap();
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
            if self.x == point_2.x && (y1_value.num + y2_value.num) == self.a.prime {
                return Ok(Point::new(self.a, self.b, None, None).unwrap());
            }

            slope = ((y2_value - y1_value).unwrap() / (x2_value - x1_value).unwrap()).unwrap();
        }

        let point_3_x = ((slope.pow(2) - x1_value).unwrap() - x2_value).unwrap();
        let point_3_y = ((slope * (x1_value - point_3_x).unwrap()).unwrap() - y1_value).unwrap();

        Ok(Point::new(self.a, self.b, Some(point_3_x), Some(point_3_y)).unwrap())
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.x == None && self.y == None {
            return write!(f, "Point(∞)_a{}_b{}", self.a, self.b);
        }

        write!(
            f,
            "Point({}, {})_a{}_b{}",
            self.x.unwrap().num,
            self.y.unwrap().num,
            self.a,
            self.b
        )
    }
}

#[cfg(test)]
mod ecc_tests {
    use super::*;

    const ORDER: u32 = 7;
    const ORDER_2: u32 = 11;

    const SECP256K1_A: FieldElement = FieldElement {
        num: 0,
        prime: ORDER,
    };
    const SECP256K1_B: FieldElement = FieldElement {
        num: 7 % ORDER,
        prime: ORDER,
    };

    #[test]
    fn test_point_init_error_infinity() {
        let x = FieldElement::new(0, ORDER_2).unwrap();
        let y = FieldElement::new(0, ORDER_2).unwrap();

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
    fn test_point_init_error_order() {
        let x = FieldElement::new(0, ORDER_2).unwrap();
        let y = FieldElement::new(0, ORDER_2).unwrap();

        assert_eq!(
            Point::new(SECP256K1_A, SECP256K1_B, Some(x), Some(y)),
            Err(format!("Cannot operate on different Fields"))
        );
    }

    #[test]
    fn test_point_init_error() {
        let x = FieldElement::new(0, ORDER).unwrap();
        let y = FieldElement::new(1, ORDER).unwrap();

        assert_eq!(
            Point::new(SECP256K1_A, SECP256K1_B, Some(x), Some(y)),
            Err(format!("coordinates ({}, {}) is not on the curve", x, y))
        );
    }

    #[test]
    fn test_point_init() {
        let x = FieldElement::new(0, ORDER).unwrap();
        let y = FieldElement::new(0, ORDER).unwrap();

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
        let x1 = FieldElement::new(1, ORDER).unwrap();
        let y1 = FieldElement::new(6, ORDER).unwrap();
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, Some(x1), Some(y1)).unwrap();

        let x2 = FieldElement::new(2, ORDER).unwrap();
        let y2 = FieldElement::new(1, ORDER).unwrap();
        let point_b = Point::new(SECP256K1_A, SECP256K1_B, Some(x2), Some(y2)).unwrap();

        assert_eq!(
            point_a + point_b,
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: Some(FieldElement::new(1, ORDER).unwrap()),
                y: Some(FieldElement::new(1, ORDER).unwrap()),
            })
        )
    }

    #[test]
    fn test_point_addition_equal() {
        let x = FieldElement::new(1, ORDER).unwrap();
        let y = FieldElement::new(6, ORDER).unwrap();
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, Some(x), Some(y)).unwrap();

        assert_eq!(
            point_a + point_a,
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: Some(FieldElement::new(2, ORDER).unwrap()),
                y: Some(FieldElement::new(6, ORDER).unwrap()),
            })
        )
    }

    #[test]
    fn test_point_addition_equal_vertical_line() {
        let x = FieldElement::new(0, ORDER).unwrap();
        let y = FieldElement::new(0, ORDER).unwrap();
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

        let x1 = FieldElement::new(1, ORDER).unwrap();
        let y1 = FieldElement::new(6, ORDER).unwrap();
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, Some(x1), Some(y1)).unwrap();

        let x2 = FieldElement::new(4, ORDER).unwrap();
        let y2 = FieldElement::new(1, ORDER).unwrap();
        let point_b = Point::new(SECP256K1_A, SECP256K1_B, Some(x2), Some(y2)).unwrap();

        assert_eq!(
            point_infinity + point_a,
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: Some(FieldElement::new(1, ORDER).unwrap()),
                y: Some(FieldElement::new(1, ORDER).unwrap()),
            })
        );

        assert_eq!(
            point_b + point_infinity,
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: Some(FieldElement::new(4, ORDER).unwrap()),
                y: Some(FieldElement::new(6, ORDER).unwrap()),
            })
        );
    }

    #[test]
    fn test_point_addition_identity_variation() {
        let x1 = FieldElement::new(1, ORDER).unwrap();
        let y1 = FieldElement::new(6, ORDER).unwrap();
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, Some(x1), Some(y1)).unwrap();

        let x2 = FieldElement::new(1, ORDER).unwrap();
        let y2 = FieldElement::new(1, ORDER).unwrap();
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
        let x = FieldElement::new(1, ORDER).unwrap();
        let y = FieldElement::new(6, ORDER).unwrap();
        let point_a = Point::new(SECP256K1_A, SECP256K1_B, Some(x), Some(y)).unwrap();

        assert_eq!(
            point_a.scalar_mul(5),
            Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x: Some(FieldElement::new(2, ORDER).unwrap()),
                y: Some(FieldElement::new(1, ORDER).unwrap()),
            }
        );
    }
}
