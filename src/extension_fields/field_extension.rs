use std::{fmt, ops};

use crate::{
    exercises::finite_field::Fp,
    extension_fields::{
        sq_root::has_root,
        utils::{gf, MODULUS},
    },
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fp2 {
    /*
     * Representation of a point that lives in an extension field: 𝔽pᵏ
     * where:
     * embedding degree `k` = 2
     * field element modulus `p` = MODULUS
     * Extension field elements: 𝔽modulus² are in the form he point is `a + bα`
     */
    pub a: Fp,
    pub b: Fp,
}

impl Fp2 {
    pub fn new(a: i64, b: i64) -> Self {
        Self { a: gf(a), b: gf(b) }
    }

    pub fn mul_inverse(self) -> Self {
        let beta = Self::select_quadratic_non_residue_beta();

        let norm = (self.a * self.a) - (self.b * self.b * beta);

        Self {
            a: self.a / norm,
            b: -self.b / norm,
        }
    }

    fn select_quadratic_non_residue_beta() -> Fp {
        for beta in 0..MODULUS {
            let specimen = gf(beta as i64);

            if beta != 0 && !has_root(&specimen) {
                return specimen;
            }
        }

        panic!("No quadratic non-residue found in Fq");
    }
}

impl ops::Add for Fp2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl fmt::Display for Fp2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "• {} + {}α — 𝔽{}",
            self.a.num, self.b.num, self.a.modulus
        )
    }
}

impl ops::Sub for Fp2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
        }
    }
}

impl ops::Mul for Fp2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.a;
        let b = self.b;
        let c = rhs.a;
        let d = rhs.b;

        let beta = Self::select_quadratic_non_residue_beta();

        Self {
            a: (a * c) + (b * d * beta),
            b: (a * d) + (b * c),
        }
    }
}

impl ops::Div for Fp2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.mul_inverse()
    }
}

impl ops::Neg for Fp2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            a: -self.a,
            b: -self.b,
        }
    }
}
