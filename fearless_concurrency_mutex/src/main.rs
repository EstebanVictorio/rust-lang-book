use std::char::from_u32;
use std::sync::{Arc, Mutex, TryLockError};
use std::thread;
use std::time::Duration;

// For this part, we will circumvent the Go programming language slogan: "do not communicate by sharing data"
// using another threading model: Mutexes
// Mutexes are a way of locking away some piece of data because a thread is using it
// And while that happens, no other thread can access it unless the lock has been freed from the underlying thread

fn main() {
    // Here we create a mutex which holds a piece of data
    // let m = Mutex::new(5);
    // {
    // Then we have to acquire the value inside with the "lock()" call which returns a Result type
    // Once we acquire it, we can unwrap it to obtain a reference to the value and mutate it
    // let mut num = m.lock().unwrap(); // MutexGuard is a smart pointer
    // *num = 6;

    // MutexGuard is a smart pointer that implements the Defer trait to our inner data
    // It also has the Drop trait implementation which is in charge of releasing the lock automatically when
    // it goes out of scope, so we don't have to worry about doing this manually!
    // }
    // Then we can print the value
    // println!("m = {:?}", m);

    // Now, we'll proceed to create a more complex example:
    // Let's say we want to include a letter for a string, which will
    // be shared among 5 dfferent threads,.by using a mutex

    // Since we have multiple pieces of data, we'll need a mutex per piece of data
    // Also,
    let letters = Arc::new(Mutex::new(vec![72, 101, 108, 108, 111]));
    let msg = Arc::new(Mutex::new(String::from("")));
    // And add a place for the handles to be joinable for the main thread to join their completion
    let mut handles = vec![];

    for i in 1..=5 {
        // Also, let's notice a key thing here:
        // The compiler would not let us get away with our stuff in terms of ownership mechanics
        // because mutexes also have to obey these rules, so, we might want multiple ownership here.
        // The important part is that, for that, we would need to use Rc, but at the same time, Rc
        // cannot be used here, because we are working with threads, which is critical for us.
        // Rc as a type does not provide thread-safe guarantees that the check ups for the references are in sync
        // among the working threads, so, we'll use a different type: Arc.
        // Arc means atomic reference count, which is a type that implements thread-safe guarantee check ups
        // which makes data vs references consistent between checks
        let letters = Arc::clone(&letters);
        let msg = Arc::clone(&msg);
        let handle = thread::spawn(move || {
            let lock = msg.lock();
            // One thing to note is that we have to be careful with deadlocks, which in simple terms is
            // where we have multiple data, (like in this case: two pices of data, a list and a string)
            // and threads lock one of the data required while other threads lock the remaining data, which, the
            // first thread that locked the data awaits for after they locked the first piece. Basically, race conditions.
            // In our program, before we even attempt to acquired, we set to acquire the second piece of data, we have to
            // acquire the first lock, so, once we're in our closure, if we acquire the first one, we're set and we can continue, since
            // the first lock acts and plays as a pre-requisite to continue execution and acquisition of the second lock.
            //. More complex programs surely can and will have deadlocks, but there are strategies that you can follow to mitigate
            // these as much as possible
            match lock {
                Ok(mut string) => {
                    let letters_lock = letters.lock();
                    let letters_list = letters_lock.unwrap();
                    let num = letters_list[i - 1];
                    println!("Num: {}", num);
                    let ascii_char = from_u32(num).unwrap();
                    println!("Char: {}", ascii_char);
                    string.push(ascii_char);
                }
                Err(_) => println!("Error: Mutex could not be acquired"),
            }
        });
        handles.push(handle);
    }

    // After all the work, one might wonder: why not having thread-safe check guarantees from the beginning?
    // The detail in that aspect is that the checks are an opt-in feature for when you "need to" do that
    // as doing the check ups deal a performance penalty on providing this safety. Sure, it might not be
    // the best, but the way we can look at it is that we'll know when we need it, otherwise, Rc is good enough for
    // when we don't need these guarantees, which is code that we know will run only in the main thread, so, we end up
    // with code that will use only what you need to use, so performance still stays in the "as best as possible" line.

    for h in handles {
        h.join().unwrap();
    }

    let mut finished = false;
    println!("Starting checkup on data...");
    while !finished {
        thread::sleep(Duration::from_secs(5));
        let msg_lock = msg.try_lock();
        match msg_lock {
            Ok(string) => {
                let letters_lock = letters.try_lock();
                match letters_lock {
                    Ok(num_letters) => {
                        if string.len() == num_letters.len() {
                            println!("Message decryption finished: {}", string);
                        }
                        finished = true;
                    }
                    Err(err) => {
                        match err {
                            TryLockError::Poisoned(_) => panic!("Error: Poisoned data from mutex!"),
                            TryLockError::WouldBlock => {
                                println!("Letters are not available at this moment, retrying in 5 secs...");
                                continue;
                            }
                        }
                    }
                }
            }
            Err(err) => match err {
                TryLockError::Poisoned(_) => panic!("Error: Poisoned data from mutex!"),
                TryLockError::WouldBlock => {
                    println!("String is not available at this moment, retrying in 5 secs...");
                }
            },
        }
    }
}
