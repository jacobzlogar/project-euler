use std::collections::HashSet;

fn main() {
    let range: Vec<usize> = (2..=20).collect();
    let res = range.into_iter().reduce(|acc, num| lcm(acc, num));
    print!("{:?}", res);
}

/// find the LCM of each pair of numbers:
/// LCM(a,b) = (a×b)/GCF(a,b)
fn lcm(first: usize, second: usize) -> usize {
    println!("first: {}", first);
    println!("second: {}", second);
    let gcf = gcf(factors(first), factors(second));
    let factor = first * second / gcf;
    println!("gcf: {}", gcf);
    println!("factor: {}", factor);
    println!("\n");
    factor
}

/// find the greatest common factor of the prime factors for each value
/// if none exist, return 1
fn gcf(first_factors: Vec<usize>, second_factors: Vec<usize>) -> usize {
    let first_set: HashSet<_> = first_factors.into_iter().collect();
    let second_set: HashSet<_> = second_factors.into_iter().collect();
    println!("{:?}", first_set);
    println!("{:?}", second_set);
    let c = first_set.intersection(&second_set).max();
    if c.is_some() {
        return *c.unwrap();
    }
    1
}

/// theres gotta be a more efficient way to do this lol
fn factors(number: usize) -> Vec<usize> {
    (2..=number)
        .rev()
        .into_iter()
        .filter_map(|n| {
            if number % n == 0 {
                return Some(n);
            }
            None
        })
        .collect()
}
