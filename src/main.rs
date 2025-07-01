use std::collections::HashMap;
// Chapter 1: Finite Fields
// Book: Programming Bitcoin: Learn How to Program Bitcoin from Scratch by Jimmy song
use std::fmt;
use std::ops;

#[derive(PartialEq)]
struct FieldElement {
    num: u32,
    prime: u32,
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
        Self {
            num: (self.num.pow(exponent)) % self.prime,
            prime: self.prime,
        }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
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

        Ok(Self {
            num: ((self.num as i64 - rhs.num as i64) % self.prime as i64) as u32,
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

fn main() {
    let a = FieldElement::new(3, 13).unwrap();
    let b = FieldElement::new(1, 13).unwrap();

    println!("{}", a.pow(3) == b);

    // Exercise 5
    let k: Vec<u64> = vec![1, 3, 7, 13, 18];
    let mut fields: HashMap<u64, Vec<u64>> = HashMap::new();
    const FIELD_ORDER: u64 = 19;

    for i in 0..k.len() {
        for j in 0..19 {
            fields
                .entry(k[i])
                .or_insert(Vec::new())
                .push((k[i] * j as u64) % FIELD_ORDER);
        }
    }

    // Exercise 7
    // let p: Vec<u64> = vec![7, 11, 17, 31];
    // let mut fields2: HashMap<u64, Vec<u64>> = HashMap::new();

    // for i in 0..p.len() {
    //     for j in 1..p[i] {
    //         fields2
    //             .entry(p[i])
    //             .or_insert(Vec::new())
    //             .push((j as u64).pow(p[i] as u32 - (1 as u32)) % p[i]);
    //     }
    // }

    // for i in 0..p.len() {
    //     println!("{:?}", fields2.get(&p[i]));
    // }
}
