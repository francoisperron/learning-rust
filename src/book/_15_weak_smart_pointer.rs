#[cfg(test)]
mod tests {
    use std::rc::{Rc, Weak};

    #[test]
    fn create_empty_weak_pointer_with_new() {
        let w: Weak<i32> = Weak::new();

        assert!(w.upgrade().is_none());
    }
    
    #[test]
    fn create_a_weak_pointer_with_downgrade_from_a_rc() {
        let p = Rc::new(5);
        let w = Rc::downgrade(&p);

        assert_eq!(*w.upgrade().unwrap(), 5);
    }

    #[test]
    fn counts_weak_refs() {
        let p = Rc::new(5);

        let _w1 = Rc::downgrade(&p);
        assert_eq!(Rc::weak_count(&p), 1);

        {
            let _w2 = Rc::downgrade(&p);
            assert_eq!(Rc::weak_count(&p), 2);
        }

        assert_eq!(Rc::weak_count(&p), 1);
    }
}