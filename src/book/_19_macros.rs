#[cfg(test)]
mod tests {
    #[macro_export]
    macro_rules! my_vec {
        // macro input is code
        // macro output is code
        // capture any value in x, than a comma, * times
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                // generate this code for each x
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    #[test]
    #[allow(clippy::vec_init_then_push)]
    fn vec_is_a_declarative_macro() {
        let a = my_vec![1, 2, 3];
        assert_eq!(a, my_vec![1, 2, 3]);
    }

    #[test]
    #[allow(clippy::vec_init_then_push)]
    // unlike fn, macros are extended at compile time
    fn unlike_fn_macros_can_accept_a_variable_number_of_params() {
        let a = my_vec![1, 2, 3];
        let b = my_vec![1, 2];

        assert_ne!(a, b);
    }

    #[test]
    fn learning_custom_derive_procedural_macros() {
        use _19_hello_macro::HelloMacro;
        use _19_hello_macro_derive::HelloMacro;

        #[derive(HelloMacro)]
        struct Jo;

        #[derive(HelloMacro)]
        struct Jack;

        assert_eq!(Jo::hello_macro(), "Hello, Macro! My name is Jo!");
        assert_eq!(Jack::hello_macro(), "Hello, Macro! My name is Jack!");
    }

    fn _learning_attributes_like_macros() {
        // #[route(GET, "/")]
        // fn index() {
        //
        // }
        //
        // #[proc_macro_attribute]
        // pub fn route(
        //     attr: TokenStream,  // contains GET and "/"
        //     item: TokenStream  // fn index
        // ) -> TokenStream {
        //
        // }
    }

    fn _learning_function_like_macros() {
        // let sql = sql!(SELECT * FROM posts WHERE id=1);
        //
        // #[proc_macro]
        // pub fn sql(input: TokenStream) -> TokenStream { }
    }
}