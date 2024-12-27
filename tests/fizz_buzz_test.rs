mod support;

// Cargo will compile each of the files in tests as an individual crate
// module is optional but helps group tests mod fizz_buzz_tests {
mod fizz_buzz_test {
    use crate::support::{given_a_multiple_of_five, given_a_multiple_of_three};
    use learning_rust::kata::fizz_buzz;

    #[test]
    fn sharing_code_between_integration_tests_using_modules() {
        let multiple_of_3 = given_a_multiple_of_three();
        let multiple_of_5 = given_a_multiple_of_five();

        assert_eq!(fizz_buzz(multiple_of_3 * multiple_of_5), "FizzBuzz");
    }
}
