use rand::Rng; // Rng trait defines methods of random number generators.
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number."); // println macro from std::fmt
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Explicit loop until continue or break.
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable variable

        io::stdin()
            .read_line(&mut guess) // Get user input and write to mutable ref.
            .expect("Failed to read line"); // Returned result is ok or err; just crash.

        let guess: u32 = match guess.trim().parse() {
            // Shadow guess variable, match
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an integer from 1-100).");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // Match against each arm pattern
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
