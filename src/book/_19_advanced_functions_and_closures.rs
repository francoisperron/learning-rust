#[cfg(test)]
mod tests {
    #[test]
    fn pass_function_pointers_to_functions() {
        let answer = do_twice(add_one, 5);

        assert_eq!(answer, (5 + 1) + (5 + 1));
    }

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    #[test]
    fn pass_function_pointers_to_map() {
        let numbers = vec![1, 2, 3];
        let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();

        assert_eq!(strings, vec!["1", "2", "3"]);
    }

    #[test]
    fn uses_enum_initializer_as_function_pointers() {
        let statuses: Vec<Status> = (0..3).map(Status::Value).collect();

        assert_eq!(statuses, vec![Status::Value(0), Status::Value(1), Status::Value(2)])
    }

    #[derive(Debug, PartialEq)]
    enum Status {
        Value(u32),
        _Stop,
    }

    #[test]
    fn returns_closure_in_a_box_to_fix_the_size_of_the_return() {
        assert_eq!(returns_closure()(5), 6);
    }

    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
