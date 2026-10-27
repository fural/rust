use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

const MIN: u32 = 1;
const MAX: u32 = 100;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(MIN..=MAX);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if !(MIN..=MAX).contains(&guess) {
            println!("Out of range!, please choose a number between {MIN} and {MAX}");
            continue;
        }

        println!("Your guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
