// Rc<T>, a reference counting type that enables multiple ownership

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn create_rc_with_new() {
        let a = Rc::new(6);

        assert_eq!(a, Rc::new(6));
    }

    #[test]
    fn access_rc_data_with_deref() {
        let a = Rc::new(6);

        assert_eq!(*a, 6);
    }

    #[test]
    fn share_rc_ownership_with_clone() {
        let a = Rc::new(6);

        let c1 = Rc::clone(&a);
        let c2 = Rc::clone(&a);

        assert_eq!(*c1, 6);
        assert_eq!(*c2, 6);
    }

    #[test]
    fn refs_are_counted_with_strong_count() {
        let a = Rc::new(6);
        assert_eq!(Rc::strong_count(&a), 1);

        {
            let _c = Rc::clone(&a);
            assert_eq!(Rc::strong_count(&a), 2);
        }

        assert_eq!(Rc::strong_count(&a), 1);
    }
}
