fn get_largest_prime(num: u64) -> u64 {
    struct Node <'a> { //TODO: will i actually end up using a BT?
        val: &'a str,
        left: Box<Option<Node<'a>>>,
        right: Box<Option<Node<'a>>>,
    }
    1
}

fn main() {
    get_largest_prime(600851475143);
}
