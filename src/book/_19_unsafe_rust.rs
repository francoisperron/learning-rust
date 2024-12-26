#[cfg(test)]
mod tests {
    #[test]
    fn allows_to_deference_a_pointer() {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        }
    }

    #[test]
    fn allows_calling_an_unsafe_function_or_method() {
        unsafe {
            assert_eq!(dangerous(), "pouet");
        }
    }

    unsafe fn dangerous() -> String { "pouet".to_string() }

    #[test]
    fn allows_implementing_an_unsafe_trait() {
        assert_eq!(3.push(), "Booh! Booh! Booh! ")
    }

    /// # Safety
    ///
    /// So strange
    unsafe trait StrangeButton {
        fn push(&self) -> String;
    }

    unsafe impl StrangeButton for usize {
        fn push(&self) -> String {
            "Booh! ".repeat(*self).to_string()
        }
    }

    #[test]
    fn allows_accessing_fields_of_a_union() {
        let n = Number { whole: 1 };
        unsafe { assert_eq!(n.whole, 1); }
    }

    union Number {
        whole: u32,
        _decimal: f32,
    }
}