#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn vectors_are_created_and_modified() {
        let vector_from_macro = vec![1, 2, 3];

        let mut vector_from_new = Vec::new();
        for element in &vector_from_macro {
            vector_from_new.push(*element);
        }
        assert_eq!(vector_from_macro, vector_from_new);

        vector_from_new[0] = 5;
        assert_ne!(vector_from_new[0], vector_from_macro[0]);
    }

    #[test]
    fn vectors_get() {
        let v = vec![1, 2, 3];

        // to get value
        let v1 = v[1];
        let v2 = v[1];
        assert_eq!(v1, v2);

        // to get references
        let r1 = &v[1];
        let r2 = &v[1];
        assert_eq!(r1, r2);

        // with get()
        let g1 = match v.get(1) {
            Some(second) => *second,
            None => 0
        };
        assert_eq!(g1, 2);
    }

    #[test]
    fn strings_are_stored_as_a_collection_of_utf8_chars() {
        let s = String::from("hello");
        let mut c = ['h', 'e', 'l', 'l', 'o'].into_iter();
        for i in s.chars() {
            assert_eq!(i, c.next().unwrap());
        }
    }

    #[test]
    fn strings_are_all_created_equal() {
        // there is no idiomatic way to create a string, and no performance difference, just pick one.
        let s1 = "s".to_string();
        let s2 = String::from("s");
        let s3 = "s".to_owned();

        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
    }

    #[test]
    fn strings_are_modified() {
        let mut s1 = "s".to_string();
        s1.push_str("os");

        assert_eq!(s1, "sos".to_string());
    }

    #[test]
    fn strings_are_joined_with_plus() {
        let s1 = "s".to_string();
        let s2 = "os".to_string();

        let s3 = s1 + &s2;  // s1 moved to s3, s1 no longer valid
        // assert_eq!(s1, "s");    // cant borrow

        assert_eq!(s3, "sos");
    }

    #[test]
    fn strings_are_joined_with_format() {
        let s1 = "s".to_string();
        let s2 = "os".to_string();

        let s3 = format!("{s1}{s2}");

        assert_eq!(s1, "s"); // can borrow s1
        assert_eq!(s3, "sos");
    }

    #[test]
    fn hashmaps_are_created() {
        let mut map: HashMap<&str, i32> = HashMap::new();
        map.insert("1", 1);
        map.insert("2", 2);

        assert_eq!(map.get("1").copied().unwrap(), 1);
    }

    #[test]
    fn hashmaps_are_created_easily_with_from() {
        let map = HashMap::from([
            ("1", 1),
            ("2", 2)
        ]);

        assert_eq!(map.get("1").copied().unwrap(), 1);
    }

    #[test]
    fn hashmaps_are_updated() {
        let mut map: HashMap<&str, i32> = HashMap::new();

        map.insert("1", 1);
        map.entry("1").or_insert(111);
        assert_eq!(map.get("1").copied().unwrap(), 1);

        map.entry("2").or_insert(2);
        assert_eq!(map.get("2").copied().unwrap(), 2);

        map.entry("3")
            .and_modify(|e| { *e += 1 })
            .or_insert(42);
        assert_eq!(map["3"], 42);

        map.entry("3")
            .and_modify(|e| { *e += 1 })
            .or_insert(555);
        assert_eq!(map["3"], 43);
    }
}