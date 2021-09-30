use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..11);
    println!("Guess the number!");
    println!("Please input your guess:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess :(");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("Your guess: {}", guess);
    println!("Secret number: {}", secret_number);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
