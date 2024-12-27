#[cfg(test)]
mod tests {
    #[test]
    fn closures_params_and_return_types_are_optional() {
        let add_one = |x| x + 1;

        assert_eq!(add_one(1), 2);
    }

    #[test]
    fn closures_can_be_passed_as_parameters() {
        let add_one = |x: i32| x + 1;
        let result = calculator_do(add_one, 1);

        assert_eq!(result, 2);
    }

    fn calculator_do(operation: fn(i32) -> i32, number: i32) -> i32 {
        operation(number)
    }

    #[test]
    fn closures_capture_variable_immutable() {
        let a = 1;
        let only_borrows = |x| x + a; // Fn

        let a1 = only_borrows(1);
        assert_eq!(a1, 2);
        assert_eq!(a, 1); // can access a
                          // a = 10;      ⚠️ a is borrowed

        let a2 = only_borrows(1);
        assert_eq!(a2, 2);
    }

    #[test]
    fn closures_capture_variable_mutable() {
        let mut a = vec![1, 2, 3];
        let mut borrows_mutably = || a.push(4); // FnMut

        borrows_mutably();
        assert_eq!(a, vec![1, 2, 3, 4]);
        // borrows_mutably(); // ⚠️assert_eq! error: can borrow as immutable
    }

    #[test]
    fn closure_can_take_ownership() {
        let a = vec![1, 2, 3];
        let takes_ownership = move |x| x == a; // FnOnce

        // assert_eq!(a, vec![1,2,3]);     // ⚠️ cant borrow a moved value

        let b = vec![1, 2, 3];
        assert!(takes_ownership(b));
    }
}
