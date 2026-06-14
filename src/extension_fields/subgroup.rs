use crate::extension_fields::utils::MODULUS;

fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1 % modulus;
    base %= modulus;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }

    result
}

fn divisors(n: u64) -> Vec<u64> {
    let mut small = vec![];
    let mut large = vec![];

    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            small.push(i);
            if i != n / i {
                large.push(n / i);
            }
        }
        i += 1;
    }

    small.extend(large.into_iter().rev());
    small
}

pub fn embedding_degree(subgroup_order: u32) -> Option<u32> {
    let p = MODULUS as u64;
    let r = subgroup_order as u64;

    if p % r == 0 {
        return None;
    }

    let phi = r - 1;
    let mut divs = divisors(phi);
    divs.sort();

    for k in divs {
        if mod_exp(p, k, r) == 1 {
            return Some(k.try_into().unwrap());
        }
    }

    None
}
