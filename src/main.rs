use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Welcome to the number guessing game!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("What is your guess?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error: Failed to read line");

        println!("You guessed {guess} ");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("To high!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
    }
}
