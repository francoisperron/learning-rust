#[allow(dead_code)]
trait Greet {
    fn greet(&self) -> String;
}

#[allow(dead_code)]
struct Alice;

impl Greet for Alice {
    fn greet(&self) -> String {
        "Hello".to_string()
    }
}

#[allow(dead_code)]
struct Carlo;

impl Greet for Carlo {
    fn greet(&self) -> String {
        "Ciao".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::learnings::dyn_box_vs_generics::{Alice, Carlo, Greet};

    #[test]
    fn struct_with_a_trait_does_not_compile() {
        // struct User {
        //     greet: Greet
        // }

        // let greet = Alice;
        // let user = User { greet };
        //                   ^^^^^ expected `dyn Greet`, found `Alice`
        //    = help: `Alice` implements `Greet` so you could box the found value and coerce it to the trait object `Box<dyn Greet>`,
        //            you will have to change the expected type as well
    }

    struct UserDyn {
        greet: Box<dyn Greet>,
    }

    #[test]
    fn struct_with_dyn_box() {
        let alice = UserDyn { greet: Box::new(Alice) };
        assert_eq!(alice.greet.greet(), "Hello");

        let carlo = UserDyn { greet: Box::new(Carlo) };
        assert_eq!(carlo.greet.greet(), "Ciao");
    }

    #[test]
    fn conditional_types_with_dyn_box() {
        let name = "carlo";

        let user = match name {
            "carlo" => UserDyn { greet: Box::new(Carlo) },
            "alice" => UserDyn { greet: Box::new(Alice) },
            _ => panic!("Unknown user"),
        };

        assert_eq!(user.greet.greet(), "Ciao");
    }

    struct UserGen<T>
    where
        T: Greet,
    {
        greet: T,
    }

    #[test]
    fn struct_with_generics() {
        let alice = UserGen { greet: Alice };
        assert_eq!(alice.greet.greet(), "Hello");

        let carlo = UserGen { greet: Carlo };
        assert_eq!(carlo.greet.greet(), "Ciao");
    }

    #[test]
    fn conditional_types_with_generics_does_not_compile() {
        // let name= "carlo";

        // let user = match name {
        //     "carlo" => UserGen { greet: Carlo },
        //     "alice" => UserGen { greet: Alice },
        //                - `match` arms have incompatible types
        //     _ => panic!("Unknown user"),
        // };
    }

    enum MyGreet {
        Alice(Alice),
        Carlo(Carlo),
    }

    impl MyGreet {
        /// Call the given closure with the appropriate [`Greet`] implementation
        fn on_greet<F, T>(&self, f: F) -> T
        where
            F: FnOnce(&dyn Greet) -> T,
        {
            match self {
                Self::Alice(v) => f(v),
                Self::Carlo(v) => f(v),
            }
        }
    }

    impl Greet for MyGreet {
        fn greet(&self) -> String {
            self.on_greet(|greet| greet.greet())
        }
    }

    #[test]
    fn wrap_generics() {
        let name = "carlo";

        let user = match name {
            "carlo" => UserGen { greet: MyGreet::Carlo(Carlo) },
            "alice" => UserGen { greet: MyGreet::Alice(Alice) },
            _ => panic!("Unknown user"),
        };

        assert_eq!(user.greet.greet(), "Ciao");
    }
}
