#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

const MAX: f64 = 1.0;
const SEVENTY_FIVE: f64 = 0.9;
const NINETY: f64 = 0.75;

pub trait Messenger {
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

        if percentage_of_max >= NINETY {
            self.messenger
                .send("Heads up: You've used 75% of your quota already! Consider slowing down.");
        }
    }
}
