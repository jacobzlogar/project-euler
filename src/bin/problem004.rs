fn main() {
    let result = get_palindromes((100, 999));
    print!("{}", result.into_iter().max().unwrap());
}

fn is_palindrome(number: usize) -> bool {
    let reversed = number.to_string().chars().rev().collect::<String>();
    reversed == number.to_string()
}

fn get_palindromes(mut range: (usize, usize)) -> Vec<usize> {
    let mut palindromes = vec![];
    'outer: while range.1 > range.0 {
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
