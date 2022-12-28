use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_2 = tx.clone();
    thread::spawn(move || {});
    thread::spawn(move || {});

    for received in rx {}
}
