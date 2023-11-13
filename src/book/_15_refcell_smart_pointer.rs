// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data;
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn create_refcell_with_new() {
        let v = RefCell::new(5);

        assert_eq!(v, RefCell::new(5));
    }

    #[test]
    fn access_refcell_with_borrow() {
        let v = RefCell::new(5);

        let c1 = v.borrow();
        let c2 = v.borrow();

        assert_eq!(*c1, 5);
        assert_eq!(*c2, 5);
    }

    #[test]
    fn change_refcell_with_borrow_mut() {
        let v = RefCell::new(5);

        let mut c1 = v.borrow_mut();
        *c1 = 10;

        assert_eq!(*c1, 10);
    }

    #[test]
    #[should_panic(expected = "already borrowed: BorrowMutError")]
    fn cant_borrow_mut_twice() {
        let v = RefCell::new(1);

        let mut one_borrow = v.borrow_mut();
        *one_borrow = 2;
        let mut two_borrow = v.borrow_mut(); // panics
        *two_borrow = 3;
    }

    #[test]
    fn combine_rc_and_refcell_to_have_multiple_owners_of_mut_data() {
        let value = Rc::new(RefCell::new(5));

        let c1 = Rc::clone(&value);
        let c2 = Rc::clone(&value);

        *value.borrow_mut() = 10;
        assert_eq!(*value.borrow(), 10);
        assert_eq!(*c1.borrow(), 10);
        assert_eq!(*c2.borrow(), 10);

        *c1.borrow_mut() = 15;
        assert_eq!(*value.borrow(), 15);
        assert_eq!(*c1.borrow(), 15);
        assert_eq!(*c2.borrow(), 15);
    }
}


