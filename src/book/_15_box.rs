// Box<T> for allocating values on the heap

#[cfg(test)]
mod tests {
    use crate::book::_15_box::tests::List::{Cons, Nil};

    #[test]
    fn create_box_with_new() {
        let b = Box::new(5); // 5 is stored on heap instead of stack

        assert_eq!(b, Box::new(5));
    }

    #[test]
    fn access_box_data_with_dereference() {
        let b = Box::new(5);

        assert_eq!(*b, 5);
    }

    #[test]
    fn enable_recursion_with_box_since_it_size_is_known_at_compile_time() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        if let Cons(v, _) = list {
            assert_eq!(v, 1);
        };
    }

    #[allow(dead_code)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
}