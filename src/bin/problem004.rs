fn main() {
    let result = get_palindromes((100, 999));
    print!("{}", result.into_iter().max().unwrap());
}

fn is_palindrome(number: u32) -> bool {
    let reversed = number.to_string().chars().rev().collect::<String>();
    reversed == number.to_string()
}

fn get_palindromes(mut range: (u32, u32)) -> Vec<u32> {
    let mut palindromes = vec![];
    while range.1 > range.0 {
        for x in (range.0..range.1).rev() {
            let product = x * range.1;
            if is_palindrome(product) {
                palindromes.push(x * range.1);
            }
        }
        range.1 -= 1;
    }

    palindromes
}
