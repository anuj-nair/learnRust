use rand::prelude::*;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input line.");
        let guess = guess.trim().parse();

        let guess: i32 = match guess {
            Result::Ok(val) => {
                if val > 100 {
                    println!("Please Enter a Integer Between 0-100");
                    continue;
                } else {
                    val
                }
            }
            _ => {
                println!("Please Enter a Integer Between 0-100");
                continue;
            }
        };

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }
}
