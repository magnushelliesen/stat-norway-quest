use std::cmp::Ordering;
use std::io;

use rand::random_bool;
use rand::random_range;

const MIN_NUMBER_OF_GUESSES: i32 = 3;
const MAX_NUMBER_OF_GUESSES: i32 = 10;
const MIN_EQUATION_NUMBER: i32 = 0;
const MAX_EQUATION_NUMBER: i32 = 15_500;

pub fn fix_model(tenure: i32) -> bool {
    let probability: f64 = 1.0 / ((tenure + 1) as f64);
    if random_bool(probability) {
        guessing_game()
    } else {
        true
    }
}

fn guessing_game() -> bool {
    println!("The model crashed, and you need to find the correct equation that failed!");

    let max_number_of_guesses: i32 = random_range(MIN_NUMBER_OF_GUESSES..MAX_NUMBER_OF_GUESSES);
    let secret_number: i32 = random_range(MIN_EQUATION_NUMBER..MAX_EQUATION_NUMBER);

    println!(
        "Guess the buggy equation number (between {MIN_EQUATION_NUMBER} and {MAX_EQUATION_NUMBER}: "
    );

    let mut counter: i32 = 0;
    let mut success: bool = false;

    while counter < max_number_of_guesses {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Handler erronous input
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input must be a number");
                continue;
            }
        };

        // Check guess against secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("No, the bug is in an equation before {guess}."),
            Ordering::Greater => println!("No, the bug is in an equation after {guess}."),
            Ordering::Equal => {
                println!("Yes, the bug is in equation {secret_number}.");
                success = true;
                break;
            }
        }

        counter += 1;
    }

    success
}
