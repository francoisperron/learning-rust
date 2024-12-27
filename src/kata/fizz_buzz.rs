pub fn fizz_buzz(number: usize) -> String {
    match (number % 3 == 0, number % 5 == 0) {
        (true, true) => "FizzBuzz".to_string(),
        (true, _) => "Fizz".to_string(),
        (_, true) => "Buzz".to_string(),
        _ => number.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_the_given_number() {
        assert_eq!(fizz_buzz(1), "1");
        assert_eq!(fizz_buzz(2), "2");
    }

    #[test]
    fn prints_fizz_for_multiple_of_3() {
        assert_eq!(fizz_buzz(3), "Fizz");
        assert_eq!(fizz_buzz(6), "Fizz");
    }

    #[test]
    fn prints_buzz_for_multiple_of_5() {
        assert_eq!(fizz_buzz(5), "Buzz");
        assert_eq!(fizz_buzz(10), "Buzz");
    }

    #[test]
    fn prints_fizzbuzz_for_multiple_of_3_and_5() {
        assert_eq!(fizz_buzz(15), "FizzBuzz");
        assert_eq!(fizz_buzz(30), "FizzBuzz");
    }
}
