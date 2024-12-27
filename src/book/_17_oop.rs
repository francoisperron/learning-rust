#[cfg(test)]
mod tests {
    #[test]
    fn oop_using_impl_struct() {
        let mut c = AveragedCollection::new();
        c.add(2);
        c.add(4);

        assert_eq!(c.average(), 3.0);
    }

    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn new() -> AveragedCollection {
            AveragedCollection { list: vec![], average: 0.0 }
        }
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }

    #[test]
    fn trait_as_generics_only_accepts_same_impl() {
        // aka static dispatch
        let screen = ScreenGenerics {
            components: vec![
                Button { label: String::from("a") },
                // TextField { text: String::from("b") }, //expected Button found TextField
            ],
        };

        assert_eq!(screen.run(), "a");
    }

    #[test]
    fn trait_as_box_dyn_accepts_all_impls() {
        // aka dynamic dispatch
        let screen = Screen { components: vec![Box::new(Button { label: String::from("a") }), Box::new(TextField { text: String::from("c") })] };

        assert_eq!(screen.run(), "ac");
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) -> String {
            self.components.iter().map(|c| c.draw()).collect()
        }
    }

    pub struct ScreenGenerics<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> ScreenGenerics<T>
    where
        T: Draw,
    {
        pub fn run(&self) -> String {
            self.components.iter().map(|c| c.draw()).collect()
        }
    }

    pub trait Draw {
        fn draw(&self) -> String;
    }

    pub struct Button {
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) -> String {
            self.label.to_string()
        }
    }

    pub struct TextField {
        pub text: String,
    }

    impl Draw for TextField {
        fn draw(&self) -> String {
            self.text.to_string()
        }
    }
}
