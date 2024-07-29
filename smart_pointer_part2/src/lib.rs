pub trait Messenger {
   fn send(&self, msg: &str); // take immutable reference of self and message string slice
}

pub struct LimitChecker<'a, T>
where
    T: Messenger,
{
    messenger: &'a T, // reference type generic must implement Messenger trait
    value: usize,
    max: usize,
}

// Here, you would define methods for LimitTracker that use or interact with T, but it doesn't
//  necessarily mean that LimitTracker itself implements the Messenger trait.
// You might use this to add functionality to LimitTracker that is independent of the Messenger trait.
// impl<'a, T> LimitTracker<'a, T>
// where
//     T: Messenger,
// {
//     // Implementation of methods for LimitTracker
// }

// This is used to make LimitTracker conform to the Messenger trait, meaning that LimitTracker
//  can now be used wherever a Messenger is expected.
// Inside the send method, you can delegate the call to self.messenger.send(msg).
// impl<'a, T> Messenger for LimitTracker<'a, T>
// where
//     T: Messenger,
// {
//     fn send(&self, msg: &str) {
//         // Implementation of the send method
//     }
// }

// The first impl block requires a trait bound (T: Messenger) only if the methods inside LimitTracker need to call methods on T that require T to implement Messenger.
// The second impl block explicitly states that LimitTracker implements Messenger for all T that also implement Messenger.

impl<'a, T> LimitChecker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitChecker<'a, T> {
        LimitChecker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
    // self.messenger is a reference to an instance of a type that implements the Messenger trait.
    // self.messenger.send calls the send method defined in the Messenger trait.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // We can create a new instance of the mock object, create a LimitTracker that uses the mock object,
    //  call the set_value method on LimitTracker, and then check that the mock object has the messages we expect

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        pub fn new(sent_messages: Vec<String>) -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, message: &str){
            println!("Hello, {}", message);
            // We can’t modify the MockMessenger to keep track of the messages, because the send method takes an immutable reference to self.
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percentage_warning_message() {
        let mock_messenger = MockMessenger::new(vec![]);
        let mut limit_tracker = LimitChecker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(),1 );
    }
}
