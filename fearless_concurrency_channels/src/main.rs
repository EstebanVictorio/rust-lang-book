use std::{sync::mpsc, thread, time::Duration};

// In this part of the concurrency chapter, we'll work with channels,
// a thread API that allows us communicate and share data through messages.

fn main() {
    // Here we technically create a channel with multiple producers and a single consumer
    // but for our purposes we'll use one producer and the single consumer
    // mpsc stands for multiple producers single consumer
    let (tx, rx) = mpsc::channel();
    let (tx_2, rx_2) = mpsc::channel();
    println!("Awaiting for receive...");

    // There are two key parts/components that play the important role here:
    // tx.send() is the method that allows us to send data through the producer
    // in this case, the String: "Hello from tx!"
    thread::spawn(move || {
        let value = String::from("channel 1: Hello from tx!");
        thread::sleep(Duration::from_millis(3000));
        tx.send(value).unwrap();
        // Now, channels are not free from ownership rules
        // if we try to use "value" after using it to send it over the channel,
        // a compile-time error will come up letting us know that it's not possible
        // to use it, say, for example, for printing the value with:
        // println!("{}", value); This is not allowed because "value" is being used and its ownership belongs to the receiver
    });

    // Then we have the rx.recv() method call, which will block the main thread until the producer has sent over some data
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Now, let's create an example that showcases that channel consumers are receiving data as the data
    // is being sent over:

    thread::spawn(move || {
        let list = vec![
            String::from("channel 2: Hello"),
            String::from("channel 2: from"),
            String::from("channel 2: channel 2"),
        ];

        for item in list {
            tx_2.send(item).unwrap();
            // Here we sleep after we send each message
            thread::sleep(Duration::from_secs(3));
        }
    });

    // Here, receiver is treated as an iterator, since it implements the Iterator trait
    // Now we don't need to call the "recv()" function explicitly, but print each received
    // message, and the main thread will block until this finishes
    for received in rx_2 {
        println!("channel 2 rx: {}", received);
    }
}
