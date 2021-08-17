#![allow(dead_code)]
fn main(input: i32) -> i32 {
    input + 1
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn test_increment() {
        assert_eq!(main(2), 3);
    }

    #[test]
    fn test_increment_birthday() {
        assert_eq!(main(19990605), 19990606);
    }

    #[test]
    fn test_not_increment() {
        assert_ne!(!main(2), 2);
    }
}
