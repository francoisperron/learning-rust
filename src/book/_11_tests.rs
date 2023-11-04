#[cfg(test)]
mod tests {
    #[test]
    fn assert_eq_works() {
        assert_eq!(1, 1);
        assert_eq!(1, 1, "{} should be {}", 1, 1);
    }
    #[test]
    fn assert_ne_works() {
        assert_ne!(1, 2);
    }

    #[test]
    fn assert_works_for_booleans() {
        assert!("hello".contains("hello"));
    }

    #[test]
    fn result_works_too() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}