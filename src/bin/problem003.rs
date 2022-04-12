use std::fmt::Debug;

#[derive(Debug,Copy,Clone)]
struct Prime {
    is_prime: bool,
    number: u64,
}

fn largest_prime_factor(num: u64) -> u64 {
    let mut primes: Vec<Prime> = vec![];

    for i in (2..num).rev() {
        let p = Prime {
            is_prime: bool::from(true),
            number: u64::from(i)
        };
        primes.push(p);
    }

    for x in &mut *primes {
        // determine_prime(&mut p);
        x.is_prime = bool::from(false);
    }

    println!("{:?}", primes);

    num
}

fn main() {
    // let prime = largest_prime_factor(600851475143);
    let prime = largest_prime_factor(120);

    println!("{}", prime);
}
