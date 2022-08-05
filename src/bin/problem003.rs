// fn is_prime(value: usize) -> bool {
//     if value == 2 || value == 3 {
//         return true
//     }

//     if value % 2 == 0 || value % 3 == 0 {
//         return false
//     }

//     let mut i: usize = 5;
//     while i * i <= value {
//         if value % i == 0 || value % (i + 2) == 0 {
//             return false
//         }
//         i += 6;
//     }

//     return true
// }
fn main() {
    let result = factors(600851475143);
    println!("{:?}", &result);
    println!("{}", result[result.len() - 1]);
}

fn is_even(value: usize) -> bool {
    value % 2 == 0
}

struct Divisor {
    value: usize
}

impl Divisor {
    fn next_factor(&self, divisible: usize) -> usize {
        let mut val: usize = self.value;
        while divisible % val != 0 {
            if is_even(val) {
                val += 1
            } else {
                val += 2
            }
        }
        val
    }
}

fn factors(mut max: usize) -> Vec<usize> {
    // start with a divisor at the smallest possible prime factor
    let initial = Divisor { value: 2 };
    // find the next divisor
    let mut divisor = Divisor { value: initial.next_factor(max) };
    let mut factors = vec![divisor.value];
    // loop until the remainder is a prime number
    while max / divisor.value != 1 {
        max = max / divisor.value;
        divisor = Divisor { value: divisor.next_factor(max) };
        factors.push(divisor.value);
    }

    factors
}
