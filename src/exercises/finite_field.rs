// Book: Programming Bitcoin: Learn How to Program Bitcoin from Scratch by Jimmy song
// Chapter 1: Finite Fields
//  ____  ____  _  _  ____  ____  ____      ____  ____  ____  __    ____   ___
// ( ___)(_  _)( \( )(_  _)(_  _)( ___)    ( ___)(_  _)( ___)(  )  (  _ \ / __)
// )__)  _)(_  )  (  _)(_   )(   )__)      )__)  _)(_  )__)  )(__  )(_) )\__ \
// (__)  (____)(_)\_)(____) (__) (____)    (__)  (____)(____)(____)(____/ (___/

use std::fmt;
use std::ops;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldElement {
    pub num: u32,
    pub modulus: u32,
}

impl FieldElement {
    pub fn new(_num: i64, _modulus: u32) -> Result<Self, String> {
        // TODO: handle `_modulus` errors
        if _modulus == 0 {
            return Err(format!("cannot define a finite field over modulus ZERO"));
        }

        let m = _modulus as i64;
        let element = ((_num % m + m) % m) as u32;

        Ok(Self {
            num: element as u32,
            modulus: _modulus,
        })
    }

    pub fn pow(self, exponent: u32) -> Self {
        let mut num: u32 = self.num;

        for _ in 0..(exponent - 1) {
            num = (num * self.num) % self.modulus;
        }

        Self {
            num: num,
            modulus: self.modulus,
        }
    }

    pub fn scalar_mul(self, by: u32) -> Self {
        let mut num: u32 = self.num;

        if by == 0 {
            num = 0;
        } else if by > 1 {
            for _ in 0..(by - 1) {
                num = (num + self.num) % self.modulus;
            }
        }

        Self {
            num: num,
            modulus: self.modulus,
        }
    }

    pub fn scalar_mul_fe(self, by: &Self) -> Self {
        self.scalar_mul(by.num)
    }

    #[allow(dead_code)]
    pub fn safe_add(self, other: Self) -> Result<Self, String> {
        if self.modulus != other.modulus {
            return Err(format!("cannot safely add two numbers in distinct fields"));
        }

        Ok(self + other)
    }

    #[allow(dead_code)]
    pub fn safe_subtract(self, other: Self) -> Result<Self, String> {
        if self.modulus != other.modulus {
            return Err(format!(
                "cannot safely subtract two numbers in distinct fields"
            ));
        }

        Ok(self - other)
    }

    #[allow(dead_code)]
    pub fn safe_mul(self, other: Self) -> Result<Self, String> {
        if self.modulus != other.modulus {
            return Err(format!(
                "cannot safely multiply two numbers in distinct fields"
            ));
        }

        Ok(self * other)
    }

    #[allow(dead_code)]
    pub fn safe_div(self, other: Self) -> Result<Self, String> {
        if self.modulus != other.modulus {
            return Err(format!(
                "cannot safely divide two numbers in distinct fields"
            ));
        }

        if other.num == 0 {
            return Err(format!("cannot divide a field element by zero"));
        }

        Ok(self / other)
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_ₚ{}({})", self.modulus, self.num)
    }
}

impl ops::Add for FieldElement {
    type Output = Self;

    // assumes `rhs` has the same MODULUS
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            num: (self.num + rhs.num) % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl ops::Sub for FieldElement {
    type Output = Self;

    // assumes `rhs` has the same MODULUS
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result: u32 = 0;

        if self.num > rhs.num {
            result = (self.num - rhs.num) % self.modulus;
        } else if self.num < rhs.num {
            result = self.modulus - (rhs.num - self.num);
        }

        Self {
            num: result,
            modulus: self.modulus,
        }
    }
}

impl ops::Mul for FieldElement {
    type Output = Self;

    // assumes `rhs` has the same MODULUS
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            num: (self.num * rhs.num) % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl ops::Div for FieldElement {
    type Output = Self;

    // assumes `rhs` has the same MODULUS & is not ZERO
    fn div(self, rhs: Self) -> Self::Output {
        let rhs_inverse = rhs.pow(self.modulus - 2);

        self * rhs_inverse
    }
}

#[cfg(test)]
mod ff_tests {
    use super::*;

    const ZERO: u32 = 0;
    const PRIME: u32 = 7;
    const PRIME_2: u32 = 11;

    #[test]
    fn test_field_element_init_error() {
        let num: i64 = PRIME as i64;

        assert_eq!(
            FieldElement::new(num, ZERO),
            Err(format!("cannot define a finite field over modulus ZERO"))
        );
    }

    #[test]
    fn test_field_element_init() {
        let num: u32 = PRIME - 1;

        assert_eq!(
            FieldElement::new(num as i64, PRIME),
            Ok(FieldElement {
                num,
                modulus: PRIME
            })
        );
    }

    #[test]
    fn test_field_element_addition_error() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME_2).unwrap();

        assert_eq!(
            a.safe_add(b),
            Err(format!("cannot safely add two numbers in distinct fields"))
        );
    }

    #[test]
    fn test_field_element_addition() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME).unwrap();

        assert_eq!(
            a + b,
            FieldElement {
                num: 1,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_subtraction_error() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME_2).unwrap();

        assert_eq!(
            a.safe_subtract(b),
            Err(format!(
                "cannot safely subtract two numbers in distinct fields"
            ))
        );
    }

    #[test]
    fn test_field_element_subtraction() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME).unwrap();

        assert_eq!(
            a - b,
            FieldElement {
                num: 5,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_multiplication_error() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME_2).unwrap();

        assert_eq!(
            a.safe_mul(b),
            Err(format!(
                "cannot safely multiply two numbers in distinct fields"
            ))
        );
    }

    #[test]
    fn test_field_element_multiplication() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME).unwrap();

        assert_eq!(
            a * b,
            FieldElement {
                num: 1,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_scalar_multiplication() {
        let a = FieldElement::new(3, PRIME).unwrap();

        assert_eq!(
            a.scalar_mul(12),
            FieldElement {
                num: 1,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_exponent() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let exponent: u32 = 4;

        assert_eq!(
            a.pow(exponent),
            FieldElement {
                num: 4,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_division_error() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME_2).unwrap();

        assert_eq!(
            a.safe_div(b),
            Err(format!(
                "cannot safely divide two numbers in distinct fields"
            ))
        );
    }

    #[test]
    fn test_field_element_division_error_zero() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let c = FieldElement::new(0, PRIME).unwrap();

        assert_eq!(
            a.safe_div(c),
            Err(format!("cannot divide a field element by zero"))
        );
    }

    #[test]
    fn test_field_element_division() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME).unwrap();

        assert_eq!(
            a / b,
            FieldElement {
                num: 2,
                modulus: PRIME
            }
        );
    }
}
