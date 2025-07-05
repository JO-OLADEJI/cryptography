// Chapter 2, 3: Elliptic Curve Cryptography (over a finite field)
// Book: Programming Bitcoin: Learn How to Program Bitcoin from Scratch by Jimmy song
use crate::exercises::ch1::FieldElement;
use std::fmt;

/* The general form of the curve is y² = x³ + ax + b, but more specifically the
 * `secp256k1` curve used by bitcoin and ethereum has the equation y² = x³ + 7
 * where `a` equals 0 and `b` equals 7
 *
 * The `Point` struct contains details that satisfy the above equation (general form)
 * over a finite field
 */
#[derive(Debug, PartialEq)]
pub struct Point {
    a: FieldElement,
    b: FieldElement,
    x: FieldElement,
    y: FieldElement,
}

impl Point {
    pub fn new(
        _a: FieldElement,
        _b: FieldElement,
        _x: FieldElement,
        _y: FieldElement,
    ) -> Result<Self, String> {
        if _a.prime != _b.prime || _b.prime != _x.prime || _x.prime != _y.prime {
            return Err(format!("Cannot operate on different Fields"));
        }

        {
            let lhs = _y.pow(2);
            let rhs_0 = _x.pow(3);
            let rhs_1 = (_a * _x).unwrap();
            let rhs_01 = (rhs_0 + rhs_1).unwrap();
            let rhs = (rhs_01 + _b).unwrap();

            if lhs != rhs {
                return Err(format!("coordinates ({}, {}) is not on the curve", _x, _y));
            }
        }

        Ok(Self {
            a: _a,
            b: _b,
            x: _x,
            y: _y,
        })
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})_a{}_b{}", self.x, self.y, self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
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
    fn test_point_init_error_order() {
        let x = FieldElement::new(0, ORDER_2).unwrap();
        let y = FieldElement::new(0, ORDER_2).unwrap();

        assert_eq!(
            Point::new(SECP256K1_A, SECP256K1_B, x, y),
            Err(format!("Cannot operate on different Fields"))
        );
    }

    #[test]
    fn test_point_init_error() {
        let x = FieldElement::new(0, ORDER).unwrap();
        let y = FieldElement::new(1, ORDER).unwrap();

        assert_eq!(
            Point::new(SECP256K1_A, SECP256K1_B, x, y),
            Err(format!("coordinates ({}, {}) is not on the curve", x, y))
        );
    }

    #[test]
    fn test_point_init() {
        let x = FieldElement::new(0, ORDER).unwrap();
        let y = FieldElement::new(0, ORDER).unwrap();

        assert_eq!(
            Point::new(SECP256K1_A, SECP256K1_B, x, y),
            Ok(Point {
                a: SECP256K1_A,
                b: SECP256K1_B,
                x,
                y
            })
        );
    }
}
