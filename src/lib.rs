extern crate rand;

use std::collections::HashMap;
use rand::Rng;

pub fn factorize(n : u64) -> String {

    let mut factor_string: String = String::new();
    let mut comp = n;
    let mut exp_map: HashMap<u64, u32> = HashMap::new();

    loop {
        let f = factor(comp);

        if !exp_map.contains_key(&f) {
            exp_map.insert(f, 1);
        } else {
            let exp = exp_map.entry(f).or_insert(0);
            *exp += 1;
        }
        if f == comp {
            break;
        }

        comp = comp / f;
    }

    let mut factors = Vec::new();

    for (base, _) in &exp_map {
        factors.push(base);
    }

    factors.sort();

    for base in factors {
        if exp_map.get(&base).is_some() {
            let exponent = exp_map.get(&base).unwrap();
            let s = format!("{}^{} ", &base, exponent);
            factor_string.push_str(&s);
        } else {
            panic!("Base among factors but does not have an exponent (This is not possible)");
        }
    }

    factor_string.trim().to_string()
}

fn factor(n : u64) -> u64 {
    let mut ret = n;

    if is_prime(ret) {
        return ret;
    }

    for i in 2..n {
        if n % i == 0 {
            ret = i;
            break;
        }
    }
    ret
}

/* left-to-right binary modular exponentiation */
fn mod_exp(base: u64, exp: u64, m: u64) -> u64 {
    let mut b = base;
    let mut e = exp;

    if m == 1 {
        return 0;
    }

    let mut r = 1;

    b = b % m;

    while e > 0 {
        if e % 2 == 1 {
            r = (r * b) % m;
        }

        e = e >> 1;
    }
    m
}

/* Miller-Rabin's Primality Test */
pub fn is_prime(p : u64) -> bool {
    if p <= 3 {
        return true;
    }

    /* p is an even number */
    if p.trailing_zeros() > 0 {
        return false;
    }

    let mut r: i32 = 0;
    let mut d: u64 = 1;

    while d * 2 <= p {
        d *= 2;
        r += 1;
    }

    d = p / d;

    /* 5 iterations to reduce risk of falsely classifying composite number as prime */
    for _ in 0..4 {
        let a = rand::thread_rng().gen_range(2, p - 2);
        let mut x = mod_exp(a, d, p);
        let mut cont: bool = false;

        if x == 1 || x == p - 1 {
            continue;
        }

        for _ in 0..r-1 {
            x = mod_exp(x, 2, p);
            if x == 1 {
                return false;
            } else if x == p - 1 {
                cont = true;
                break;
            }
            return false;
        }
        if cont {
            continue;
        }
        return true;
    }
    false
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn primes_under_500() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
            73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
            173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269,
            271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487,
            491, 499];

        for p in &primes {
            println!("{}", p);
            assert!(is_prime(*p));
        }
    }

    #[test]
    fn squares() {
        assert_eq!(build_factor_string(9), "3^2");
        assert_eq!(build_factor_string(25), "5^2");
        assert_eq!(build_factor_string(64), "8^2");
    }

    #[test]
    fn power_of_twos() {
        assert_eq!(build_factor_string(2), "2^1");
        assert_eq!(build_factor_string(4), "2^2");
        assert_eq!(build_factor_string(8), "2^3");
        assert_eq!(build_factor_string(16), "2^4");
        assert_eq!(build_factor_string(32), "2^5");
        assert_eq!(build_factor_string(64), "2^6");
        assert_eq!(build_factor_string(128), "2^7");
    }
}
