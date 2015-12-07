use std::io;
use std::collections::HashMap;

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

fn is_prime(p : u64) -> bool {
    let mut ret = p;

    for i in 2..p {
        if p % i == 0 {
            ret = i;
            break;
        }
    }
    if ret == p {
        true
    } else {
        false
    }
}
