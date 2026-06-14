use crate::{exercises::finite_field::FieldElement, extension_fields::utils::gf};

// Tonelli-Shanks algorithm - assumes field has a prime order
// Returns Some(x) such that x^2 ≡ n (mod p), or None if no solution
pub fn tonelli_shanks(specimen: &FieldElement) -> Option<u64> {
    let n = specimen.num as u64;
    let p = specimen.modulus as u64;

    if n == 0 {
        return Some(0);
    }
    if p == 2 {
        return Some(n);
    }

    // Check if solution exists
    if legendre_symbol(n, p) != 1 {
        return None;
    }

    // Factor p - 1 = q * 2^s
    let mut q = p - 1;
    let mut s = 0;

    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    // Find a quadratic non-residue z
    let mut z = 2;
    while legendre_symbol(z, p) != p - 1 {
        z += 1;
    }

    let mut c = mod_pow(z, q, p);
    let mut t = mod_pow(n, q, p);
    let mut r = mod_pow(n, (q + 1) / 2, p);

    let mut m = s;

    while t != 1 {
        let mut i = 1;
        let mut t2i = (t as u128 * t as u128 % p as u128) as u64;

        while t2i != 1 {
            t2i = (t2i as u128 * t2i as u128 % p as u128) as u64;
            i += 1;

            if i == m {
                return None;
            }
        }

        let b = mod_pow(c, 1 << (m - i - 1), p);

        r = ((r as u128 * b as u128) % p as u128) as u64;
        t = ((t as u128 * b as u128 % p as u128) * b as u128 % p as u128) as u64;
        c = ((b as u128 * b as u128) % p as u128) as u64;

        m = i;
    }

    Some(r)
}

pub fn has_root(value: &FieldElement) -> bool {
    if *value == gf(0) {
        return true;
    }

    let power = gf(value.modulus as i64 - 1) / gf(2);

    value.pow(power.num) == gf(1)
}

fn mod_pow(base: u64, mut exp: u64, p: u64) -> u64 {
    let mut result: u128 = 1;
    let mut base = base as u128;
    let p = p as u128;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % p;
        }
        base = (base * base) % p;
        exp >>= 1;
    }

    result as u64
}

fn legendre_symbol(a: u64, p: u64) -> u64 {
    let ls = mod_pow(a % p, (p - 1) / 2, p);

    ls
}
