#[cfg(test)]
mod tests {
    #[test]
    fn pattern_matching_value() {
        let (x, y, z) = (1, 2, 3);

        assert_eq!(x, 1);
        assert_eq!(y, 2);
        assert_eq!(z, 3);
    }

    #[test]
    fn pattern_matching_param() {
        let point = (1, 2);
        let result = print_coordinates(&point);

        assert_eq!(result, "(1, 2)");
    }

    fn print_coordinates(&(x, y): &(i32, i32)) -> String {
        format!("({}, {})", x, y)
    }

    #[test]
    fn irrefutable_and_refutable_patterns() {
        // irrefutable pattern = will match for all values
        let x = 5;
        assert_eq!(x, 5);

        // refutable pattern = fails to match for some value [E0005]
        // pattern `None` not covered
        // let Some(x) = Some(5);

        if let Some(y) = Some(5) {
            assert_eq!(y, 5);
        }
    }

    #[test]
    fn match_literal_works() {
        assert_eq!(match_literal(1), "one");
        assert_eq!(match_literal(4), "anything");
    }

    fn match_literal(x: u32) -> &'static str {
        match x {
            1 => "one",
            2 => "two",
            3 => "three",
            _ => "anything",
        }
    }

    #[test]
    fn match_named_variable_works() {
        assert_eq!(match_named_variable(Some(50)), "50!!");
        assert_eq!(match_named_variable(Some(77)), "77");
        assert_eq!(match_named_variable(None), "0");
    }

    fn match_named_variable(x: Option<u32>) -> String {
        match x {
            Some(50) => "50!!".to_string(),
            Some(y) => format!("{y}"),
            _ => "0".to_string(),
        }
    }

    #[test]
    fn match_multiple_patterns() {
        assert_eq!(match_multiple(1), "a little");
        assert_eq!(match_multiple(6), "all good");
    }

    fn match_multiple(x: u32) -> &'static str {
        match x {
            1 | 2 => "a little",
            3 | 4 => "some",
            5..=10 => "all good",
            _ => "too much",
        }
    }

    #[test]
    fn match_tuple_like_array() {
        let numbers = (2, 4, 8, 16, 32);

        let (first, .., last) = numbers;
        assert_eq!(first, 2);
        assert_eq!(last, 32);
    }

    #[test]
    fn destructing_struct_works() {
        let p = Point { x: 1, y: 2 };

        let Point { x: a, y: b } = p;
        assert_eq!(a, 1);
        assert_eq!(b, 2);

        let Point { x, y } = p;
        assert_eq!(x, 1);
        assert_eq!(y, 2);
    }

    #[test]
    fn match_struct_works() {
        assert_eq!(match_struct(Point { x: 1, y: 0 }), "On the x axis at 1");
        assert_eq!(match_struct(Point { x: 0, y: 2 }), "On the y axis at 2");
        assert_eq!(match_struct(Point { x: 4, y: 2 }), "4 in range 1..10");
        assert_eq!(match_struct(Point { x: 11, y: 2 }), "On neither axis: (11, ..)");
    }

    fn match_struct(p: Point) -> String {
        match p {
            Point { x, y: 0 } => format!("On the x axis at {x}"),
            Point { x: 0, y } => format!("On the y axis at {y}"),
            Point{ x: x_range @ 1..=10, .. } => format!("{x_range} in range 1..10"),
            Point { x, .. } => format!("On neither axis: ({x}, ..)"),
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn destructing_enum_works() {
        assert_eq!(destructing_enum(Message::Quit), "The Quit variant has no data to destructure.");
        assert_eq!(destructing_enum(Message::Move { x: 1, y: 2 }), "Move in the x direction 1 and in the y direction 2");
        assert_eq!(destructing_enum(Message::Write("blah".to_string())), "Text message: blah");
        assert_eq!(destructing_enum(Message::ChangeColor(0, 160, 255)), "Change the color to red 0, green 160, and blue 255");
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn destructing_enum(msg: Message) -> String {
        match msg {
            Message::Quit => "The Quit variant has no data to destructure.".to_string(),
            Message::Move { x, y } => format!("Move in the x direction {x} and in the y direction {y}"),
            Message::Write(text) => format!("Text message: {text}"),
            Message::ChangeColor(r, g, b) => format!("Change the color to red {r}, green {g}, and blue {b}"),
        }
    }

    #[test]
    fn match_slices_works() {
        assert_eq!(match_slices(&["hello", "world", "!"]), "Hello World!");
        assert_eq!(match_slices(&["Foo", "Bar", "Foo", "Bar"]), "Baz");
        assert_eq!(match_slices(&["Foo", "!"]), "!!!");
        assert_eq!(match_slices(&["cric", "crac", "croc", "z"]), "starts with [\"cric\", \"crac\", \"croc\"]");
        assert_eq!(match_slices(&["a", "b"]), "ends with [\"b\"]");
    }

    fn match_slices(words: &[&str]) -> String {
        match words {
            ["hello", "world", "!", ..] => "Hello World!".to_string(),
            ["Foo", "Bar", ..] => "Baz".to_string(),
            [.., "!"] => "!!!".to_string(),
            [start @ .., "z"] => format!("starts with {:?}", start),
            ["a", end @ ..] => format!("ends with {:?}", end),
            rest => format!("{:?}", rest),
        }
    }

    #[test]
    fn match_array_works() {
        assert_eq!(match_array(&[1, 2, 3]), "starts with one");
        assert_eq!(match_array(&[1, 2]), "starts with 1, ends with 2");
        assert_eq!(match_array(&[2, 3, 4]), "starts with 2, ends with 4");
        assert_eq!(match_array(&[3]), "");
    }

    fn match_array(arr: &[i32]) -> String {
        match arr {
            [1, _, _] => "starts with one".to_string(),
            [a, .., b] => format!("starts with {a}, ends with {b}"),
            _ => "".to_string()
        }
    }

    #[test]
    fn match_vector_works() {
        assert_eq!(match_vector(&vec![1, 2]), "12:2");
        assert_eq!(match_vector(&vec![1, 2, 3]), "123:3");
        assert_eq!(match_vector(&vec![1, 2, 3, 4, 5]), "15:?");
        assert_eq!(match_vector(&vec![1]), "");
    }

    fn match_vector(vec: &Vec<i32>) -> String {
        match vec[..] {
            [a, b] => format!("{a}{b}:2"),
            [a, b, c] => format!("{a}{b}{c}:3"),
            [a, .., b] => format!("{a}{b}:?"),
            _ => "".to_string()
        }
    }

    #[test]
    fn matches_macro_works() {
        let v = [1,2,3];
        assert!(std::matches!(v, [1, ..]));
    }
}