use std::boxed::Box;
use std::error::Error as Err;
use std::fs::{read_to_string, File};
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

// Functions have return types as we've seen, but, the "main()" function is a special function
// that indicates through its return value what was the ultimate result of a program
// Like in C, Rust has defined values for each use case, and as in C, 0 is the way to indicate
// that a program completed successfully. If otherwise, the "main" function will end with some other error code.
// To indicate this, we can signature the return type of the "main" function with a "Result<(), Box<dyn Err>>", indicating
// that the function will return a Result type with either the empty tuple or any encountered error in the process
fn main() -> Result<(), Box<dyn Err>> {
    // Using the macro "panic!" we can make our program go into an error state, without anything further to do
    // panic!("Oops!");

    /** Here we have an example which will panic from trying to access a location which does not belong to the vector */
    /** In Rust, this panics to prevent vulnerabilities to come up */
    /** The panic! location is inside the vector module where an out of bounds index is attempted */
    /** If we attempt to run this code with the `RUST_BACKTRACE=1` env var set,
     * it will give you the exact location and number of functions that were
     * executed, even if the functions are not precisely located in this code, but in modules used in this code, like
     * the vec! macro. This follow-up of functions is known as backtrace, and, as the name says, it traces
     * the functions that were run back to its origin. */
    let vector = vec![1, 2, 3];
    // println!("{}", vector[99]);

    // Another way to handle errors is to use the Result enum type
    // This type will wrap the actual result of the operation we wanted to perform, like, opening a file:
    let file_result = File::open("hello.txt");
    match file_result {
        Ok(_) => println!("File opened!"),
        // Here we can match pattern the result type to gracefully let the user know if there was an error and print its
        // message afterwards
        Err(error) => match error.kind() {
            // We can also acknowledge which kind of error happened with the ErrorKind type,
            // matching it again and printing more useful information and how to achieve fixes
            ErrorKind::NotFound => match File::create("hello.txt") {
                // One solution is to create the file if it's not found
                Ok(_) => println!("File created!"),
                // And print the error if creation was not possible at all
                Err(error_msg) => println!("Error creating file: {:?}", error_msg),
            },
            error_msg => println!("Unknown error: {:?}", error_msg),
        },
    }

    // We can shorten the example by using ".unwrap_or_else()" from the Result type so we can add defaults when an error happens
    let file_result_unwrap = File::open("hello.txt").unwrap_or_else(|error| {
        // If the unwrap does not succeed and ends up in the Error variant,
        // we can handle it by defining a default behavior
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {}", error);
            })
        } else {
            panic!("Error opening file: {}", error);
        }
    });

    unwrap_and_expect();

    //  let propagation_result = propagate_if_cannot_read_file();
    // if let Ok(username) = propagation_result {
    //     println!("Username: {}", username);
    //     return;
    // }
    // panic!(
    //     "Error reading username: {}",
    //     propagation_result.err().unwrap()
    // )

    // let propagation_result = simplified_propagate();
    // if let Ok(username) = propagation_result {
    //     println!("Username: {}", username);
    //     return;
    // }
    // panic!(
    //     "Error reading username: {}",
    //     propagation_result.err().unwrap()
    // )

    // let propagation_result = even_simpler_propagation();
    // if let Ok(username) = propagation_result {
    //     println!("Username: {}", username);
    //     return;
    // }
    // panic!(
    //     "Error reading username: {}",
    //     propagation_result.err().unwrap()
    // )

    let propagation_result = way_simpler_one_liner();
    if let Ok(username) = propagation_result {
        println!("Username: {}", username);
        return Ok(());
    }
    panic!(
        "Error reading username: {}",
        propagation_result.err().unwrap()
    );

    // Keep in mind that the question mark operator can only be used in a
    // function that returns either "Result" or "Option", or a type that implements the "FromResidual" trait
    // The behavior for the Option type is that, the Some variant from the operation is used to continue,
    // and the None value is used as an early return
    // A demonstration of this is the following example usage of a function that
    // reads some chars if available from a text. This function will attempt to get the last char if available:
    let last_char_from_word = last_char_from_text("Super");
    // TODO: finish printing this ".unwrap()" of the Option type example for the question mark operator
    println!("Last char: {}", last_char_from_word.unwrap());
}

fn last_char_from_text(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn unwrap_and_expect() {
    // ".expect()" works very similar to ".unwrap()", except that it allows you
    // to choose which will be the error message at the "panic!()" if an error happens when attempting to open the file
    let _file_result = File::open("hello.txt").expect("Error opening file \"hello.txt\"!");
}

/** A way to propagate errors is to, instad of handling them where they occur, just return a Result with the error
 * variant from an operation and let the caller handle accordingly
 */
fn propagate_if_cannot_read_file() -> Result<String, Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}

/** As you can see, the "propagate_if_cannot_read_file()" is a little bit verbose by
 * using match pattern.
 * We can write a concise and better way of the same operations using the question mark operator
 * The question mark operator will return early the Err variant of the Result type if it happens in your code
 * Otherwise, it will continue doing whatever it was doing.
 * There's a differente though, which is the following: by using the question mark operator,
 * the Error type passes implicitly into the ".from()" method from the "From" trait.
 * This is to convert the error type received from the operation to the one defined in your function.
 * This is useful because, if the "From" trait has a way to convert it into the error type you defined, it will,
 * which gives a meaningful way to represent the error context you might fall into, and then the function
 * will ultimately end up returning the converted error type from the one that occurred in your code.
 */
fn simplified_propagate() -> Result<String, Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/** We can even chain question mark statements into a single one to make our example simpler */
fn even_simpler_propagation() -> Result<String, Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

/** There's even a simpler way to write the read from file and fill a string with its contents.
 * Since reading from a file is a very common operation in Rust, there's a function that does that already:
 * ".read_to_string()" from the "fs" module.
 */
fn way_simpler_one_liner() -> Result<String, Error> {
    read_to_string("username.txt")
}
