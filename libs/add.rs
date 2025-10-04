pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Test with built-in test framework
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(34, 35), 69);
    }

    #[test]
    fn test_add_neg() {
        assert_ne!(add(34, 35), 2137);
    }

    #[test]
    fn test_wrong() {
        assert_eq!(add(34, 35), -123);
    }
}
