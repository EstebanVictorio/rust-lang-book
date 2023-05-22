use std::thread;
use std::time::Duration;

// In this lesson we check fearless concurrency, a feature of Rust that ensures no subtle concurrency bugs are introduced
// into the main codebase at compile-time

fn main() {
    // For the first example, we'll use threads, which also implement the 1:1 model
    // for threads, which use one operating system thread for each language-created thread
    // in the execution of your program

    // Here we'll try to print a sequence of numbers in the main thread and in a spawn()'ed
    // thread

    // The execution for the thread will continue until the main thread has finished.
    // What this means for us is that the spawned thread will run until the main thread finishes
    // It's like the main thread takes the lead and doesn't wait for other threads to execute:
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("Thread number: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if we move the "join()" call here, the main thread will await for the thread to run before it executes its
    // "for" block

    handle.join().unwrap();

    for i in 1..=5 {
        println!("Main Thread number: {}", i);
        // Both calls to sleep() will force the current thread in context to sleep for the given duration
        // in both cases we are sleeping the thread for 1 millisecond
        thread::sleep(Duration::from_millis(1));
    }

    // The order in which these execute don't have a guarantee regarding the order
    // Execution is completely dependant on the operating system, which might differ from other computers
    // It could also be that the main thread does not give a chance to the spawned thread to execute at all!

    // Thankfully, the thread API provies with something called handles, which use the join thread execution model
    // that blocks the current thread on a "join()" call to not perform any further work until the thread represented
    // by the handle finishes execution:

    // handle.join().unwrap();
    // in the call above this line, was to demonstrate the effect of making the main thread "await" another thread's execution completion

    // Another scenario in threads is the shared usage of data

    // One example we have is sharing a variable:

    let list = vec![1, 2, 3];

    // here in the following example we'll have a compile error on the usage of list, because the thread is attempting to use a value which the compiler
    // doesn't know if it will outlive the program's execution

    // let handle_2 = thread::spawn(|| {
    //     for e in list.iter() {
    //         println!("Item: {}", e);
    //     }
    // });

    // to overcome and force the ownership of the data around the closure, we should use the "move" reserved word
    // this will in turn move the ownership of the environment data, such as "list" into the closure, and now
    // the thread can freely consume the data there

    let handle_2 = thread::spawn(move || {
        for e in list.iter() {
            println!("Item: {}", e);
        }
    });

    handle_2.join().unwrap();

    // another scenario that's more plausible is that the data used in the thread
    // could be dropped before it even gets to execute.
    // The "move" reserved word mechanism forces the ownership to move inside the closure
    // so its usage after it ensures that the ownership for such data is correct and does not allow
    // compiling if the thread owns a certain piece of data
    // This also means that we ensure that at compile time, we cannot drop the data before the thread gets to use it
}
