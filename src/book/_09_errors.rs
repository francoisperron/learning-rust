#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Error, Read};

    #[test]
    #[should_panic(expected = "omg")]
    fn panic_is_unrecoverable() {
        std::panic::set_hook(Box::new(|_| {})); // remove backtrace print

        panic!("omg");
    }

    #[test]
    fn result_enum_represents_success_or_failure() {
        let file_open = File::open("./src/book/_09_error_hello.txt");
        let file = match file_open {
            Ok(file) => Some(file),
            Err(_) => None,
        };

        assert_eq!(file.is_some(), true);
    }

    #[test]
    fn errors_can_be_propagated_with_question_mark() -> Result<(), Error> {
        let content = read_file("./src/book/_09_error_hello.txt")?;

        assert_eq!(content, "hello");
        Ok(())
    }

    fn read_file(filename: &str) -> Result<String, Error> {
        let mut content = String::new();
        File::open(filename)?.read_to_string(&mut content)?;
        Ok(content)
    }
}
