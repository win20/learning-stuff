use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::{io, println};

fn main() {
    println!("Guess the number! {}", "Enter q to quit".blue());
    println!("Please input your guess");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut number_of_guesses = 0;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().eq("q") {
            break;
        } else {
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            number_of_guesses += 1;
            println!();
            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Too low!".red()),
                Ordering::Greater => println!("{}", "Too high!".red()),
                Ordering::Equal => {
                    println!("{}", "You win!".green());
                    println!("You made {} guesses", number_of_guesses);
                    break;
                }
            }
        }
    }
}
