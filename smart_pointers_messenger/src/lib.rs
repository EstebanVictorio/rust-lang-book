#[cfg(test)]
// Here we have the test module that will implement a Messenger trait which will serve
// as the mock messenger implementation and will hold RefCells so we can simulate
// the Messenger implementation as if we would be using them in a real case scenario
// RefCells excel here. We could use references or mut references to mutate the state
// of such values, but, Rust does not have built-in objects as in other languages, so,
// we'll use a struct which can play such role of an object. But, since structs need
// a lifetime paramenter for references, we cannot match that from the Messenger trait.
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
        // sent_messages_2: Vec<String>,
        value: RefCell<usize>,
    }

    impl MockMessenger {
        fn new() -> Self {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
                // sent_messages_2: vec![],
                value: RefCell::new(0),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            println!("Message sent: {}", msg);
            self.sent_messages.borrow_mut().push(String::from(msg)); // When using "borrow_mut()" on a RefCell, we get a smart pointer of type RefMut<T>
                                                                     // self.sent_messages_2.push(String::from(msg)); <-- NOTE: if the send method had a mut ref allowed from the Messenger trait,
                                                                     //this would be allowed, although we may not want to do this in an environment where we'd like to keep a context where immutable values are the only ones allowed
            *self.value.borrow_mut() = 5;
            println!("Value assigned: {}", self.value.borrow()); // When using "borrow()" on a RefCell, we get a smart pointer of type Ref<T>

            // Both Ref<T> and RefMut<T> smart pointer types implement the Deref trait, so, we can use them as regular references
            // RefCell keeps count of how many owners of each type we have, either multiple immutable smart pointer references or if we exceeded
            // the RefMut<T> smart pointer reference limit, which is, only one. And if the latter were to happen, rather than having a compiler error,
            // we would have a panic because this is ensured only at runtime, not at compiling time.
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = Tracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        assert_eq!(*mock_messenger.value.borrow(), 5);
    }
}

const MAX: f64 = 1.0;
const SEVENTY_FIVE: f64 = 0.75;
const NINETY: f64 = 0.9;

pub trait Messenger {
    // fn send(&mut self, msg: &str); <-- NOTE: if the self reference was mutable, we could hold the messages in the Messenger implementation
    fn send(&self, msg: &str);
}

pub struct Tracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> Tracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Tracker<'a, T> {
        Tracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= MAX {
            self.messenger.send(
                "Warning! You've used all of your quota already! Billing has potentially started",
            );

            return;
        }

        if percentage_of_max >= NINETY {
            self.messenger.send(
                "Slight Warning: You've used 90% of your quota already! Consider slowing down.",
            );

            return;
        }

        if percentage_of_max >= SEVENTY_FIVE {
            self.messenger
                .send("Heads up: You've used 75% of your quota already! Consider slowing down.");
        }
    }
}
