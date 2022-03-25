fn get_multiples(num: i32, max: i32) -> Vec<i32> {
    let mut start = num;
    let mut results = vec![];
    while start < max {
        if start % num == 0 {
            results.push(start);
        }
        start += 1;
    }

    results
}

fn main() {
    let multiples_of_three: i32 = get_multiples(3, 1000).iter().sum();
    let multiples_of_five: i32 = get_multiples(5, 1000).iter().sum();
    let multiples_of_fifteen: i32 = get_multiples(15, 1000).iter().sum();
    let total: i32 = (multiples_of_three + multiples_of_five) - multiples_of_fifteen;
    println!("multiples of three: {}", multiples_of_three);
    println!("multples of five: {}", multiples_of_five);
    println!("total: {}", total);
}
