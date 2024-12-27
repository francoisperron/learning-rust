/// Adds one to the number given.
///
/// # Examples
///
/// ```
///
/// use learning_rust::add_one;
///
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use crate::book::_14_doctest::add_one;

    #[test]
    fn doc_test_runs_with_tests() {
        assert_eq!(add_one(1), 2);
    }
}
