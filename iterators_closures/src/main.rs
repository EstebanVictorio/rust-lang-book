use iterators_closures::cacher::Cacher;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_random_number: u8 = rand::random();
    println!("Choose intensity: ");
    let mut intensity_input = String::new();
    io::stdin()
        .read_line(&mut intensity_input)
        .expect("Invalid input");
    let desired_intensity: u8 = intensity_input
        .trim()
        .parse::<u8>()
        .expect("Invalid input number");
    generate_workout(desired_intensity, simulated_random_number);
    // let x = vec![1, 2, 3];
    // let func = move |z| z == x;
    // let y = vec![1, 2, 3];
    // println!("X usage: {:?}", x);
    // func(y);
}

// For this lesson on closures and iterators, we are simulating an app that calculates the workout for
// two given inputs: a random number and a user input
// First we wrote a function to perform a simulated calculation at the top level
// Then we've used it along the "generate_workout()" function
// It ended up in repetitive code that we used in every required case
// And if we wanted to keep a non-verbose, simple and effective function,
// we'd perform the calculation before and then storing the result in a variable
// Which we'll use in any of the if clauses below it
// fn simulated_expensive_calculation(intensity: u8) -> u8 {
//     println!("calculating slowly...");
//     let cap_intensity = intensity as u64;
//     thread::sleep(Duration::from_secs(cap_intensity));
//     intensity
// }

fn generate_workout(intensity: u8, random_number: u8) {
    // We ended up creating a closure, which is an anonymous function
    // that can have arguments which are not necessarily typed as the compiler
    // can extract enough information from the usages of its parameters.
    // If you attempt to mix its argument usage types, the compiler will throw
    // an error and the code won't compile at all.
    // This closure basically does the same as the
    // "simulated_expensive_calculation()" function
    // But, in this example, we implement a Cacher structure and impl
    // to memoize the value and return the result of the expensive calculation
    // if it's used or required again
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        let cap_intensity = num as u64;
        thread::sleep(Duration::from_secs(cap_intensity));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
        return;
    }

    if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated");
        return;
    }

    println!(
        "Today, run for {} minutes!",
        expensive_closure.value(intensity)
    )
}
