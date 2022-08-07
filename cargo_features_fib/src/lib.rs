#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_checks_position_fourth() {
        let fourth_element = fib(4);
        println!("Fourth element: {}", fourth_element);
        assert_eq!(3, fourth_element);
    }

    #[test]
    fn it_checks_position_fifth() {
        let fifth_element = fib(5);
        println!("Fifth element: {}", fifth_element);
        assert_eq!(5, fifth_element);
    }

    #[test]
    fn it_checks_position_sixth() {
        let sixth_element = fib(6);
        println!("Sixth element: {}", sixth_element);
        assert_eq!(8, sixth_element);
    }
}

// To publish a crate to crates.io, we'll do a simple fibonacci sequence position calculator
// function
// TODO: Publish to crates.io
pub fn fib(pos: u64) -> u64 {
    let limits = [0, 1];
    if limits.contains(&pos) {
        return pos;
    }

    return fib(pos - 1) + fib(pos - 2);
}
