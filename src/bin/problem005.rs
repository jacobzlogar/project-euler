use std::collections::HashMap;
#[derive(Clone, Debug)]
struct Factor {
    number: u32,
    factor: u32,
    exponent: u32,
}

fn main() {
    let factors: Vec<Factor> = (2..=20)
        .into_iter()
        .filter_map(|n| factorization(n))
        .map(|(number, factor, exponent)| Factor {
            number,
            factor,
            exponent,
        })
        .collect();

    // filter out the duplicate factors retaining only the entry with the largest exponent
    // e.g: if we have (8,2,3) (4,2,2) (2,2,1) and (16,2,4) remove all but the last
    let mut exponents: HashMap<u32, u32> = HashMap::new();
    for factor in factors {
        exponents
            .entry(factor.factor)
            .and_modify(|e| {
                *e = factor.exponent.max(*e);
            })
            .or_insert(factor.exponent);
    }
    println!("{:?}", exponents);

    let value = exponents
        .into_iter()
        .fold(1, |acc, (f, e)| multiply_by_exponent(acc, (f, e)));

    dbg!(value);
}

fn multiply_by_exponent(mut acc: u32, factor: (u32, u32)) -> u32 {
    for _ in 0..factor.1 {
        acc = acc * factor.0;
    }
    acc
}

fn factorization(number: u32) -> Option<(u32, u32, u32)> {
    // if the number is prime thats the only factor
    if is_prime(number) {
        return Some((number, number, 1));
    }

    // extract the prime factors of the number
    let factors: Vec<u32> = (2..=number)
        .into_iter()
        .filter(|n| is_prime(*n) && n <= &(number / 2))
        .collect();

    // if the number can be created with a prime factor raised exponentially:
    // return a tuple containing the number, the prime factor and the exponent
    for factor in factors.clone() {
        let mut carry = factor;
        let mut exponent = 1;
        while carry < number {
            carry *= factor;
            exponent += 1;
            if carry == number {
                return Some((number, factor, exponent));
            }
        }
    }
    None
}

fn is_prime(value: u32) -> bool {
    if value == 2 || value == 3 {
        return true;
    }

    if value % 2 == 0 || value % 3 == 0 {
        return false;
    }

    let mut i: u32 = 5;
    while i * i <= value {
        // 5, 7 ... 11, 13 ... 17, 19 ... 23, 25
        if value % i == 0 || value % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}
