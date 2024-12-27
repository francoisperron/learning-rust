#[cfg(test)]
mod tests {

    #[test]
    fn new_types_pattern_helps_specify_type() {
        let age = Age(46);
        assert_eq!(age, Age(46));
    }

    #[derive(Debug, PartialEq)]
    struct Age(usize);

    #[test]
    fn type_aliases_to_name_a_type() {
        let m: Meters = 2;
        let u: usize = 2;

        assert_eq!(m, u);
    }

    type Meters = usize;

    #[test]
    fn type_aliases_to_simplify_complex_type() {
        let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("f"));

        assert_eq!(takes_long_type(f), "long");
        returns_long_type()();

        let f2: Simple = Box::new(|| println!("f2"));
        assert_eq!(takes_simple_type(f2), "long");
        returns_simple_type()();
    }

    fn takes_long_type(_f: Box<dyn Fn() + Send + 'static>) -> String {
        "long".to_string()
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        Box::new(|| println!("long return"))
    }

    type Simple = Box<dyn Fn() + Send + 'static>;

    fn takes_simple_type(_f: Simple) -> String {
        "long".to_string()
    }

    fn returns_simple_type() -> Simple {
        Box::new(|| println!("long return"))
    }

    fn _never_type_means_that_function_will_never_returns() -> ! {
        loop {
            println!("loops forever");
        }
    }
}
