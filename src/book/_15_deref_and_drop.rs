#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    #[test]
    fn deref_trait_allows_to_dereference() {
        let x = 5;
        let a = SmartPointer::new(x);

        assert_eq!(*a, 5); // works because of impl of Deref
        assert_eq!(*(a.deref()), 5); // what is called by Rust
    }

    #[test]
    fn deref_mut_works_too() {
        let x = 5;
        let mut a = SmartPointer::new(x);

        *a = 6; // works because of impl of DerefMut
        assert_eq!(*a, 6);
    }

    #[test]
    fn rust_has_automatic_deref_coercion() {
        let s = SmartPointer::new(String::from("hello"));
        let is_hello = |x: &str| assert_eq!(x, "hello");

        is_hello(&s); // &SmartPointer<String> -> &String -> &str
        is_hello(&(*s)[..]); // double deref like this
    }

    #[test]
    fn drop_trait_is_called_when_variable_goes_out_of_scope() {
        println!("before scope");
        {
            let _s = SmartPointer::new(1);
            let _s = SmartPointer::new(2);
        } // drop are printed here
        println!("out of scope");
    }

    // tuple struct of one element
    struct SmartPointer<T>(T);

    impl<T> SmartPointer<T> {
        fn new(x: T) -> SmartPointer<T> {
            SmartPointer(x)
        }
    }

    impl<T> Deref for SmartPointer<T> {
        type Target = T; // associated type

        fn deref(&self) -> &T {
            &self.0 // first item in the tuple
        }
    }

    impl<T> DerefMut for SmartPointer<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    impl<T> Drop for SmartPointer<T> {
        fn drop(&mut self) {
            println!("drop");
        }
    }

    #[test]
    fn auto_deref_on_vec() {
        let friends = Friends { friends: vec![Friend { name: "bil".to_string() }, Friend { name: "bo".to_string() }] };

        assert_eq!(call(&friends.friends), "hello bil, hello bo");
    }

    struct Friends {
        friends: Vec<Friend>,
    }

    struct Friend {
        name: String,
    }

    fn call(friends: &[Friend]) -> String {
        friends
            .iter()
            .map(|f| format!("hello {}", f.name))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
