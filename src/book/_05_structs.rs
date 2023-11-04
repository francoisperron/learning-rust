#[cfg(test)]
mod tests {
    #[test]
    fn lets_use_a_struct() {
        let batman = User { active: true, username: String::from("batman"), email: String::from("someone@example.com"), sign_in_count: 0 };
        let batman_clone = User { active: true, username: String::from("batman"), email: String::from("someone@example.com"), sign_in_count: 0 };

        assert_eq!(batman, batman_clone);
    }

    #[test]
    fn mutates_a_struct() {
        let mut batman = User { active: true, username: String::from("batman"), email: String::from("someone@example.com"), sign_in_count: 1000 };
        batman.sign_in_count = 0;

        assert_eq!(batman.sign_in_count, 0);
    }

    #[test]
    fn struct_updates_syntax_copies_data() {
        let mut batman = User { active: true, username: String::from("batman"), email: String::from("someone@example.com"), sign_in_count: 1000 };
        let robin = User { username: String::from("robin"), ..batman };

        batman.email = String::from("private@batcave.com");

        assert_eq!(robin.username, "robin");
        assert_eq!(robin.email, "someone@example.com");
        assert_eq!(batman.email, "private@batcave.com");
    }

    #[test]
    fn struct_can_have_method_using_impl() {
        let mut batman = User { active: true, username: String::from("batman"), email: String::from("someone@example.com"), sign_in_count: 0 };
        batman.signed_in();

        assert_eq!(batman.sign_in_count, 1);
    }

    #[test]
    fn struct_can_have_static_method_aka_associated_function() {
        let new_user = User::new(String::from("any name"), String::from("any mail"));

        assert_eq!(new_user.username, "any name");
    }

    #[derive(PartialEq, Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    impl User {
        fn signed_in(&mut self) {
            self.sign_in_count += 1;
        }
    }

    impl User {
        fn new(username: String, email: String) -> User {
            User { active: false, username, email, sign_in_count: 0 }
        }
    }
}