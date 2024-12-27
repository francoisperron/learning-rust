#[cfg(test)]
mod tests {
    #[test]
    fn learning_traits_with_associated_type() {
        let mut c = Counter {};
        assert_eq!(c.next(), Some(0));
    }

    pub trait Iterator {
        type Item;
        // associated type
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {}

    impl Iterator for Counter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            Some(0)
        }
    }

    // cant have another implementation for another associated type
    // impl Iterator for Counter {
    //     type Item = isize;
    //
    //     fn next(&mut self) -> Option<Self::Item> {
    //         Some(0)
    //     }
    // }

    #[test]
    fn learning_traits_with_generics() {
        let mut c1 = Counter {};
        assert_eq!(c1.next_g(), Some(0usize));
        assert_eq!(c1.next_g(), Some(-1isize));
    }

    pub trait IteratorG<T> {
        fn next_g(&mut self) -> Option<T>;
    }

    impl IteratorG<usize> for Counter {
        fn next_g(&mut self) -> Option<usize> {
            Some(0)
        }
    }

    impl IteratorG<isize> for Counter {
        fn next_g(&mut self) -> Option<isize> {
            Some(-1)
        }
    }

    #[test]
    fn learning_default_trait_parameters_type() {
        assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
    }

    use std::fmt;
    use std::fmt::{Display, Formatter};
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        // other is by default of type Self
        fn add(self, other: Point) -> Point {
            Point { x: self.x + other.x, y: self.y + other.y }
        }
    }

    #[test]
    fn another_example() {
        let m = Meters(1);
        let mm = Millimeters(234);

        assert_eq!(mm + m, Millimeters(1234))
    }

    #[derive(Debug, PartialEq)]
    struct Millimeters(u32);

    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    #[test]
    fn using_super_traits() {
        let p = Point { x: 1, y: 2 };
        assert_eq!(p.outline_print(), "**(1,2)**")
    }

    trait Print {
        fn to_string(&self) -> String;
    }

    trait OutlinePrint: Print {
        fn outline_print(&self) -> String {
            format!("**{}**", self.to_string())
        }
    }

    impl Print for Point {
        fn to_string(&self) -> String {
            format!("({},{})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    #[test]
    fn use_new_type_pattern_to_implement_trait_on_struct_outside_crate_aka_orphan_rule() {
        let v = vec![String::from("hello"), String::from("world")];
        // println!("v = {}", v); // v does not implement display
        assert_eq!(Wrapper(v).to_string(), "[hello, world]");
    }

    struct Wrapper(Vec<String>);

    impl Display for Wrapper {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
}
