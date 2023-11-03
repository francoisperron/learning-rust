#[cfg(test)]
mod tests {
    #[test]
    fn use_simple_enums() {
        let easy = Difficulty::Easy;
        let hard = Difficulty::Hard;

        assert_ne!(easy, hard);
    }

    #[derive(PartialEq, Debug)]
    enum Difficulty {
        Easy,
        Medium,
        Hard,
    }

    #[test]
    fn enums_can_store_data() {
        let ipv4 = IpAddr::V4(127, 0, 0, 1);
        let ipv6 = IpAddr::V6(String::from("::1"));

        assert_eq!(ipv4.is_ipv4(), true);
        assert_eq!(ipv6.is_ipv4(), false);
    }

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    impl IpAddr {
        fn is_ipv4(&self) -> bool {
            match self {
                IpAddr::V4(_, _, _, _) => true,
                IpAddr::V6(_) => false
            }
        }
    }

    #[test]
    fn options_are_nice() {
        let x = 5;
        let y = Some(5);
        let z: Option<i32> = None;

        let sum = x + y.unwrap_or(0);
        assert_eq!(sum, 10);

        let sum2 = x + z.unwrap_or(0);
        assert_eq!(sum2, 5);
    }

    #[test]
    fn if_let_syntax_for_only_one_case_of_match() {
        let some_difficulty = Difficulty::Medium;

        let mut is_medium = false;
        if let Difficulty::Medium = some_difficulty {
            is_medium = true;
        }

        assert_eq!(is_medium, true);
    }
}