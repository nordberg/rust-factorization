extern crate rand;

use std::io;
use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut vec = Vec::new();

    loop {
        let mut composite = String::new();
        io::stdin().read_line(&mut composite)
            .ok()
            .expect("Failed to read input");

        let number: u64 = match composite.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Failed to parse"),
        };

        if number == 0 {
            break;
        }

        vec.push(number);
    }

    for x in &vec {
        println!("{}", build_factor_string(*x));
    }
}

fn build_factor_string(n : u64) -> String {

    let factor_string: String = "".to_owned();
    let mut comp = n;
    let mut factors: HashMap<u64, u32> = HashMap::new();

    loop {
        let f = factor(comp);

        if !factors.contains_key(&f) {
            factors.insert(f, 1);
        } else {
            let exp = factors.entry(f).or_insert(0);
            *exp += 1;
        }
        if f == comp {
            break;
        }

        comp = comp / f;
    }

    for (base, exp) in factors {
        print!("{}^{} ", base, exp);
    }

    factor_string.to_string()
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

fn is_prime(p : u64) -> bool {
    if p <= 3 {
        return true;
    }

    if p.trailing_zeros() > 0 {
        return false;
    }

    let mut pr = p;
    let mut r: i32 = 0;
    let mut d: u64 = 1;

    while d * 2 <= p {
        d *= 2;
        r += 1;
    }

    d = p / d;

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
