// Book: Programming Bitcoin: Learn How to Program Bitcoin from Scratch by Jimmy song
// Chapter 1: Finite Fields
//  ____  ____  _  _  ____  ____  ____      ____  ____  ____  __    ____   ___
// ( ___)(_  _)( \( )(_  _)(_  _)( ___)    ( ___)(_  _)( ___)(  )  (  _ \ / __)
// )__)  _)(_  )  (  _)(_   )(   )__)      )__)  _)(_  )__)  )(__  )(_) )\__ \
// (__)  (____)(_)\_)(____) (__) (____)    (__)  (____)(____)(____)(____/ (___/

use std::fmt;
use std::ops;

use crate::exercises::ec_point::Field;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Fp {
    pub num: u32,
    pub modulus: u32,
}

impl Fp {
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

    #[allow(dead_code)]
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

impl Field for Fp {
    fn zero(modulus: u32) -> Self {
        Self::new(0, modulus).unwrap()
    }

    fn one(modulus: u32) -> Self {
        Self::new(1, modulus).unwrap()
    }

    fn modulus(&self) -> u32 {
        self.modulus
    }

    fn pow(self, exponent: u32) -> Self {
        let normalized_exponent = exponent % self.modulus;
        let mut num: u32 = self.num;

        if exponent == 0 {
            return Self {
                num: 1,
                modulus: self.modulus,
            };
        }

        for _ in 0..(normalized_exponent - 1) {
            num = (num * self.num) % self.modulus;
        }

        Self {
            num: num,
            modulus: self.modulus,
        }
    }

    fn mul_inverse(self) -> Self {
        self.pow(self.modulus - 2)
    }

    fn add_inverse(self) -> Self {
        Self {
            num: self.modulus - self.num,
            modulus: self.modulus,
        }
    }

    fn is_zero(self) -> bool {
        self.num == 0
    }

    fn scalar_mul(self, by: u32) -> Self {
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
}

impl fmt::Display for Fp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} — 𝔽{}", self.num, self.modulus)
    }
}

impl ops::Add for Fp {
    type Output = Self;

    // assumes `rhs` has the same MODULUS
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            num: (self.num + rhs.num) % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl ops::Sub for Fp {
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

impl ops::Mul for Fp {
    type Output = Self;

    // assumes `rhs` has the same MODULUS
    fn mul(self, rhs: Self) -> Self::Output {
        self.scalar_mul(rhs.num)
    }
}

impl ops::Div for Fp {
    type Output = Self;

    // assumes `rhs` has the same MODULUS & is not ZERO
    fn div(self, rhs: Self) -> Self::Output {
        let rhs_inverse = rhs.pow(self.modulus - 2);

        self * rhs_inverse
    }
}

impl ops::Neg for Fp {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.num == 0 {
            return self;
        }

        Self {
            num: self.modulus - self.num,
            modulus: self.modulus,
        }
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
            Fp::new(num, ZERO),
            Err(format!("cannot define a finite field over modulus ZERO"))
        );
    }

    #[test]
    fn test_field_element_init() {
        let num: u32 = PRIME - 1;

        assert_eq!(
            Fp::new(num as i64, PRIME),
            Ok(Fp {
                num,
                modulus: PRIME
            })
        );
    }

    #[test]
    fn test_field_element_addition_error() {
        let a = Fp::new(3, PRIME).unwrap();
        let b = Fp::new(5, PRIME_2).unwrap();

        assert_eq!(
            a.safe_add(b),
            Err(format!("cannot safely add two numbers in distinct fields"))
        );
    }

    #[test]
    fn test_field_element_addition() {
        let a = Fp::new(3, PRIME).unwrap();
        let b = Fp::new(5, PRIME).unwrap();

        assert_eq!(
            a + b,
            Fp {
                num: 1,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_subtraction_error() {
        let a = Fp::new(3, PRIME).unwrap();
        let b = Fp::new(5, PRIME_2).unwrap();

        assert_eq!(
            a.safe_subtract(b),
            Err(format!(
                "cannot safely subtract two numbers in distinct fields"
            ))
        );
    }

    #[test]
    fn test_field_element_subtraction() {
        let a = Fp::new(3, PRIME).unwrap();
        let b = Fp::new(5, PRIME).unwrap();

        assert_eq!(
            a - b,
            Fp {
                num: 5,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_multiplication_error() {
        let a = Fp::new(3, PRIME).unwrap();
        let b = Fp::new(5, PRIME_2).unwrap();

        assert_eq!(
            a.safe_mul(b),
            Err(format!(
                "cannot safely multiply two numbers in distinct fields"
            ))
        );
    }

    #[test]
    fn test_field_element_multiplication() {
        let a = Fp::new(3, PRIME).unwrap();
        let b = Fp::new(5, PRIME).unwrap();

        assert_eq!(
            a * b,
            Fp {
                num: 1,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_scalar_multiplication() {
        let a = Fp::new(3, PRIME).unwrap();

        assert_eq!(
            a.scalar_mul(12),
            Fp {
                num: 1,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_exponent() {
        let a = Fp::new(3, PRIME).unwrap();
        let exponent: u32 = 4;

        assert_eq!(
            a.pow(exponent),
            Fp {
                num: 4,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_zero_exponent() {
        let a = Fp::new(4, PRIME).unwrap();
        let exponent: u32 = 0;

        assert_eq!(
            a.pow(exponent),
            Fp {
                num: 1,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_one_exponent() {
        let a = Fp::new(6, PRIME).unwrap();
        let exponent: u32 = 1;

        assert_eq!(
            a.pow(exponent),
            Fp {
                num: 6,
                modulus: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_exponent_overflow() {
        let a = Fp::new(3, PRIME).unwrap();
        let exponent: u32 = 12;
        let normalized_exponent = 12 % PRIME;

        let computed_power = a.pow(exponent);
        let expected_power = a.pow(normalized_exponent);

        assert_eq!(computed_power, expected_power);
    }

    #[test]
    fn test_field_element_division_error() {
        let a = Fp::new(3, PRIME).unwrap();
        let b = Fp::new(5, PRIME_2).unwrap();

        assert_eq!(
            a.safe_div(b),
            Err(format!(
                "cannot safely divide two numbers in distinct fields"
            ))
        );
    }

    #[test]
    fn test_field_element_division_error_zero() {
        let a = Fp::new(3, PRIME).unwrap();
        let c = Fp::new(0, PRIME).unwrap();

        assert_eq!(
            a.safe_div(c),
            Err(format!("cannot divide a field element by zero"))
        );
    }

    #[test]
    fn test_field_element_division() {
        let a = Fp::new(3, PRIME).unwrap();
        let b = Fp::new(5, PRIME).unwrap();

        assert_eq!(
            a / b,
            Fp {
                num: 2,
                modulus: PRIME
            }
        );
    }
}
