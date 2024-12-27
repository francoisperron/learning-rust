#[cfg(test)]
mod tests {
    #[test]
    fn if_else_works() {
        let condition = true;
        let number = if condition { 5 } else { 6 };
        assert_eq!(number, 5);
    }

    #[test]
    fn for_range_works() {
        let mut expected_number = [3, 2, 1].into_iter();
        for number in (1..4).rev() {
            assert_eq!(number, expected_number.next().unwrap());
        }
    }

    #[test]
    fn const_needs_to_be_typed() {
        const ANYTHING: f64 = 1.23456;
        assert_eq!(ANYTHING, 1.23456);

        const DB: &str = "localhost:8080";
        assert_eq!(DB, "localhost:8080");
    }

    #[test]
    fn mutable_variable_works() {
        let mut x = 5;
        assert_eq!(x, 5);
        x = 6;
        assert_eq!(x, 6);
        x = 100_000;
        assert_eq!(x, 100000);
    }

    #[test]
    fn immutable_variable_works() {
        let x = 3;
        //x = 4;
        assert_eq!(x, 3);
    }

    #[test]
    fn tuple_works() {
        let tup = (500, 6.4, 1, "banana");

        let (_, b_by_destructor, _, _) = tup;
        assert_eq!(b_by_destructor, 6.4);

        let b_by_index = tup.1;
        assert_eq!(b_by_index, 6.4);
    }
}
