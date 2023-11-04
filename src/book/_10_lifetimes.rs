#[cfg(test)]
mod tests {
    #[test]
    fn generic_lifetime_annotation_describes_relationship_between_lifetimes_of_multiple_references() {
        let s1 = String::from("very_long");
        let s2 = String::from("short");

        let result = longest(s1.as_str(), s2.as_str());
        assert_eq!(result, s1);
    }

    #[test]
    fn return_value_has_the_lifetime_of_the_smallest_params_lifetime() {
        let _s1 = String::from("very_long");

        let _result: &str;
        {
            let _s2 = String::from("short");
            // _result = longest(_s1.as_str(), _s2.as_str()); // s2 does not live long enough E0597
        }
        // assert_eq!(_result, _s1);
    }

    // lifetime annotation defined with <'a> (lifetime annotation needs to start with ')
    // annotating x, y, and the return value with the same lifetime annotation does not change x and y lifetimes
    // it means that the lifetime of the return value will be smallest lifetime of x and y
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    #[test]
    fn structs_can_hold_references_with_lifetimes() {
        let s = StructWithAReference { the_ref: "oh lala" };
        assert_eq!(s.the_ref, "oh lala");
    }

    struct StructWithAReference<'a> {
        the_ref: &'a str,
        // part2: &str, // missing lifetime specifier E0106
    }

    // lifetime elision rules
    // lifetimes on parameters are called input lifetimes
    // lifetimes on return values are called output lifetimes.

    // 1. Each reference parameter gets its own input lifetime
    // 2. If there is only one input lifetime, then input lifetime is assigned to all output lifetimes
    // 3. If there are multiple input lifetimes, and one of them is &self or &mut self, then that input lifetime is assigned to all output lifetimes
    // Otherwise, lifetimes need to be manually assigned

    #[test]
    fn impl_on_struct_with_lifetime() {
        let s = StructWithAReference { the_ref: "oh lala" };
        assert_eq!(s.with_ending("!"), "oh lala!");
    }

    impl<'a> StructWithAReference<'a> {
        fn with_ending(&self, ends: &str) -> String {
            format!("{text}{ends}", text=self.the_ref)
        }
    }

    #[test]
    fn static_lifetime_can_live_as_long_as_the_program() {
        let s: &'static str = "I have a static lifetime.";
        assert_eq!(s, "I have a static lifetime.");
    }
}