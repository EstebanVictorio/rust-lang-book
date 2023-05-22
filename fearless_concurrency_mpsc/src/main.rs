use std::{sync::mpsc, thread, time::Duration};

// Jumping into another example, now we'll take advantage of the multiple producers feature.
// We can call "clone()" on the transmitter part of a channel so it has multiple ways
// to produce data to the receiver

fn main() {
    // Here we'll send messages from two threads using the original transmitter and a clone
    let (tx, rx) = mpsc::channel();
    let tx_2 = tx.clone();
    thread::spawn(move || {
        let list = vec![
            String::from("tx 1: Hello"),
            String::from("tx 1: from"),
            String::from("tx 1: tx "),
            String::from("tx 1: 1"),
        ];

        for msg in list {
            thread::sleep(Duration::from_secs(3));
            tx.send(msg).unwrap();
        }
    });

    thread::spawn(move || {
        let list = vec![
            String::from("tx 2: Hello"),
            String::from("tx 2: from"),
            String::from("tx 2: tx "),
            String::from("tx 2: 2"),
        ];

        for msg in list {
            thread::sleep(Duration::from_secs(3));
            tx_2.send(msg).unwrap();
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
