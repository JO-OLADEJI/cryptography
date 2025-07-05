// Chapter 1: Finite Fields
// Book: Programming Bitcoin: Learn How to Program Bitcoin from Scratch by Jimmy song
use std::fmt;
use std::ops;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldElement {
    pub num: u32,
    pub prime: u32,
}

impl FieldElement {
    pub fn new(_num: u32, _prime: u32) -> Result<Self, String> {
        if _num >= _prime {
            return Err(format!("Num {} not in field range 0 to {}", _num, _prime,));
        }

        Ok(Self {
            num: _num,
            prime: _prime,
        })
    }

    pub fn pow(self, exponent: u32) -> Self {
        let mut num: u32 = self.num;

        for _ in 0..(exponent - 1) {
            num = (num * self.num) % self.prime;
        }

        Self {
            num: (self.num.pow(exponent)) % self.prime,
            prime: self.prime,
        }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_ₚ{}({})", self.prime, self.num)
    }
}

impl ops::Add for FieldElement {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return Err(format!("Cannot add two numbers in different Fields"));
        }

        Ok(Self {
            num: (self.num + other.num) % self.prime,
            prime: self.prime,
        })
    }
}

impl ops::Sub for FieldElement {
    type Output = Result<Self, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(format!("Cannot subtract two numbers in different Fields"));
        }
        let mut result: u32 = 0;

        if self.num > rhs.num {
            result = (self.num - rhs.num) % self.prime;
        } else if self.num < rhs.num {
            result = self.prime - (rhs.num - self.num);
        }

        Ok(Self {
            num: result,
            prime: self.prime,
        })
    }
}

impl ops::Mul for FieldElement {
    type Output = Result<Self, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(format!("Cannot multiply two numbers in different Fields"));
        }

        Ok(Self {
            num: (self.num * rhs.num) % self.prime,
            prime: self.prime,
        })
    }
}

impl ops::Div for FieldElement {
    type Output = Result<Self, String>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(format!("Cannot divide two numbers in different Fields"));
        }

        if rhs.num == 0 {
            return Err(format!("Cannot divide a Field element by zero"));
        }

        let rhs_inverse = rhs.pow(self.prime - 2);

        self * rhs_inverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIME: u32 = 7;
    const PRIME_2: u32 = 11;

    #[test]
    fn test_field_element_init_error() {
        let num: u32 = PRIME;

        assert_eq!(
            FieldElement::new(num, PRIME),
            Err(format!("Num {} not in field range 0 to {}", num, PRIME))
        );
    }

    #[test]
    fn test_field_element_init() {
        let num: u32 = PRIME - 1;

        assert_eq!(
            FieldElement::new(num, PRIME),
            Ok(FieldElement { num, prime: PRIME })
        );
    }

    #[test]
    fn test_field_element_addition_error() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME_2).unwrap();

        assert_eq!(
            a + b,
            Err(format!("Cannot add two numbers in different Fields"))
        );
    }

    #[test]
    fn test_field_element_addition() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME).unwrap();

        assert_eq!(
            a + b,
            Ok(FieldElement {
                num: 1,
                prime: PRIME
            })
        );
    }

    #[test]
    fn test_field_element_subtraction_error() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME_2).unwrap();

        assert_eq!(
            a - b,
            Err(format!("Cannot subtract two numbers in different Fields"))
        );
    }

    #[test]
    fn test_field_element_subtraction() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME).unwrap();

        assert_eq!(
            a - b,
            Ok(FieldElement {
                num: 5,
                prime: PRIME
            })
        );
    }

    #[test]
    fn test_field_element_multiplication_error() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME_2).unwrap();

        assert_eq!(
            a * b,
            Err(format!("Cannot multiply two numbers in different Fields"))
        );
    }

    #[test]
    fn test_field_element_multiplication() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME).unwrap();

        assert_eq!(
            a * b,
            Ok(FieldElement {
                num: 1,
                prime: PRIME
            })
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
                prime: PRIME
            }
        );
    }

    #[test]
    fn test_field_element_division_error() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME_2).unwrap();

        assert_eq!(
            a / b,
            Err(format!("Cannot divide two numbers in different Fields"))
        );
    }

    #[test]
    fn test_field_element_division_error_zero() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let c = FieldElement::new(0, PRIME).unwrap();

        assert_eq!(a / c, Err(format!("Cannot divide a Field element by zero")));
    }

    #[test]
    fn test_field_element_division() {
        let a = FieldElement::new(3, PRIME).unwrap();
        let b = FieldElement::new(5, PRIME).unwrap();

        assert_eq!(
            a / b,
            Ok(FieldElement {
                num: 2,
                prime: PRIME
            })
        );
    }
}
