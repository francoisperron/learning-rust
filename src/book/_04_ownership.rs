#[cfg(test)]
mod tests {
    #[test]
    fn stack_is_fix() {
        // stack 1->* stack frame 1->1 executed function 1->* local variables of function
        print_stack("before test");
        let (x, y, z) = a();
        print_stack("after test");

        assert_eq!(x, "hello");
        assert_eq!(y, 22);
        assert_eq!(z, "world")
    }

    fn a() -> (&'static str, i32, String) {   // creates a frame on stack
        print_stack("a");
        let x = "hello";                    // create a reference on stack to a string in binary
        let y = 22;                          // stored on stack
        let z = b();                       // creates a frame on stack
        return (x, y, z);                     // remove frame from stack
    }

    fn b() -> String {                       // creates a frame on stack
        print_stack("b");
        let x = "world".to_string();      // allocates memory on the heap for the string
        // and stores the pointer to the heap on stack
        return x;                            // remove frame from stack
    }

    fn print_stack(info: &str) {
        match stacker::remaining_stack() {
            Some(value) => println!("{value} : {info}"),
            None => println!("unable to get remaining stack")
        }
    }

    #[test]
    fn ownership_rules() {
        // 1. Each value has an owner.
        // 2. There can only be one owner at a time.
        // 3. When the owner goes out of scope, the value will be dropped.
        {
            let _s = String::from("bonjour");
        } // scope ends, the value is dropped, the memory on the heap in deallocated
    }

    #[test]
    fn copies_data_on_stack() {
        let mut a = "1";
        let b = a; // a copied into b
        assert_eq!(a, b);

        a = "3";
        assert_eq!(a, "3");
        assert_eq!(b, "1");
    }

    #[test]
    fn function_call_copies_like_variable_assignation() {
        let a = 1;
        copies_value(a);                 // a was copied into i variable, a still valid
        assert_eq!(a, 1);
    }

    fn copies_value(i: i32) {
        assert_eq!(i, 1);
    }

    #[test]
    fn moves_data_on_heap() {
        let a = String::from("hello"); // a points to hello on heap
        let _b = a;
        // b also points to hello on heap
        // a has moved into b
        // Rust considers a as no longer valid

        // assert_eq!(a, b);                     // ⚠️borrow of moved value a error E0382

        let c = String::from("world");
        let d = c.clone();
        assert_eq!(c, d);
    }

    #[test]
    fn function_call_moves_like_variable_assignation() {
        let a = String::from("hello");
        takes_ownership(a);                 // a has moved into s variable, a no longer valid
        // println!("{a}")                     // ⚠️borrow of moved value a error E0382
    }

    fn takes_ownership(s: String) {
        println!("{s}")
    }                                           // s is dropped

    #[test]
    fn function_return_moves_returned_value() {
        let s = gives_ownership();      // a has moved into s
        assert_eq!(s, "bye");                  // s is usable
    }

    fn gives_ownership() -> String {
        let a = String::from("bye");
        return a;
    }

    #[test]
    fn borrowing_to_keep_ownership() {
        let a = String::from("hello");
        let len = calculate_length(&a); // pass a reference to a to keep ownership aka borrowing

        assert_eq!(a, "hello");
        assert_eq!(len, 5);
    }

    fn calculate_length(s: &String) -> usize { // s points to a that points to hello
        let length = s.len();
        length
    } // s is dropped, but not a


    #[test]
    fn use_mutable_reference_to_allow_changes() {
        let mut a = "Hello".to_string();
        change(&mut a);

        assert_eq!(a, "Hello world");
    }

    fn change(s: &mut String) {
        s.push_str(" world");
    }

    #[test]
    fn cannot_have_more_than_one_mutable_reference() {
        let mut a = "Hello".to_string();
        let _b = &mut a;
        // let c = &mut a;         // ⚠️ cannot borrow a as mutable more that once E0499
        // assert_eq!(b, c);
    }

    #[allow(unused_mut)]
    #[test]
    fn cannot_mix_mutable_and_immutable_references() {
        let mut a = "Hello".to_string();
        let _b = &a;
        // let c = &mut a;         // ⚠️ cannot borrow a as mutable  E0502
        // assert_eq!(b, c);
    }

    #[test]
    fn references_are_dropped_at_last_use() {
        let mut a = "Hello".to_string();
        let b = &a;

        assert_eq!(b, "Hello");

        let c = &mut a;
        assert_eq!(c, "Hello");     // can declare a mutable reference to a since b is dropped
    }
}