use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..11);
    loop {
        println!("Guess the number!");
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess :(");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Your guess: {}", guess);
                println!("Too small!")
            }
            Ordering::Greater => {
                println!("Your guess: {}", guess);
                println!("Too big!")
            }
            Ordering::Equal => {
                println!("Your guess: {}", guess);
                println!("Secret number: {}", secret_number);
                println!("You win!");
                break;
            }
        }
    }
}
