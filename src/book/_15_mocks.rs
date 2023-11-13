pub trait Messenger {
    fn send(&self, msg: &str);
}

#[allow(dead_code)]
pub struct LimitTracker<'a> {
    messenger: &'a dyn Messenger,
    value: usize,
    max: usize,
}

impl<'a> LimitTracker<'a> {
    #[allow(dead_code)]
    pub fn new(messenger: &'a impl Messenger, max: usize) -> LimitTracker<'a> {
        LimitTracker { messenger, value: 0, max }
    }

    #[allow(dead_code)]
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        let message = match percentage_of_max {
            v if v >= 1.0 => Some("Error: You are over your quota!"),
            v if v >= 0.9 => Some("Urgent warning: You've used up over 90% of your quota!"),
            v if v >= 0.75 => Some("Warning: You've used up over 75% of your quota!"),
            _ => None
        };

        if let Some(m) = message {
            self.messenger.send(m);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

    #[test]
    fn sends_an_over_75_percent_warning_message() {
        let messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&messenger, 10);
        limit_tracker.set_value(8);

        assert_eq!(messenger.sent_messages.borrow()[0], "Warning: You've used up over 75% of your quota!");
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // use the interior immutability pattern ...
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // ... to access mut sent_message using immutable &self
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
}