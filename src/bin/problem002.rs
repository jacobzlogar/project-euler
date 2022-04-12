fn get_fib_sum(num: u64, max: u64) -> u64 {
    let mut fibs = vec![num];
    let first: u64 = num + num;
    fibs.push(first);

    let mut last: u64 = *fibs.last().unwrap();

    while last < max {
        let second_to_last = &fibs[fibs.len() - 2..];
        let new = second_to_last.first().unwrap() + last;
        last = new;
        if last < max {
            fibs.push(last);
        }
    }

    println!("{:?}",  fibs);

    fibs.iter().filter(|x| *x % 2 == 0).sum()
}

fn main() {
    print!("{}", get_fib_sum(1, 4000000));
}
