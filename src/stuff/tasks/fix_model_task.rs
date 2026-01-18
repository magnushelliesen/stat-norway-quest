use std::cmp::Ordering;
use std::io;

use rand::random_bool;
use rand::random_range;

const MIN_NUMBER_OF_GUESSES: i32 = 3;
const MAX_NUMBER_OF_GUESSES: i32 = 15;

const MAX_EQUATION_NUMBER: i32 = 15_500;

pub fn fix_model(tenure: i32) -> bool {
    let probability: f64 = 1.0 / ((tenure + 2) as f64);
    if random_bool(probability) {
        guessing_game()
    } else {
        true
    }
}

fn guessing_game() -> bool {
    let min_equation_range: i32 = random_range(1..MAX_EQUATION_NUMBER - 1);
    let max_equation_range: i32 = random_range(min_equation_range..MAX_EQUATION_NUMBER);

    let max_number_of_guesses: i32 = random_range(MIN_NUMBER_OF_GUESSES..MAX_NUMBER_OF_GUESSES);
    let secret_number: i32 = random_range(min_equation_range..max_equation_range);

    let mut counter: i32 = 0;
    let mut success: bool = false;

    println!(
        "\nOh crap, the model crashed...\n\
        You need to find the correct equation causes the crash.\n\
        The deadline is right around the corner, so you have only {max_number_of_guesses} attempts. \n\n\
        Guess the buggy equation number \
        (you have reason to believe that it's somewhere between equation number {min_equation_range} and {max_equation_range})..."
    );

    while counter < max_number_of_guesses {
        let mut guess = String::new();
        let remaining_guesses: i32 = max_number_of_guesses - counter;

        println!("\nTake a guess (you have {remaining_guesses} remaining guesses): ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Handler erronous input
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your guess must be a number, you dum-dum...");
                counter += 1;
                continue;
            }
        };

        // Handle guesses outside range
        if guess < min_equation_range || guess > min_equation_range {
            println!("Your guess is outside the outside the expeced range.\nYou wasted precious time, now that's really stupid...");
            break;
        }

        // Check guess against secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("No, the bug is in an equation after {guess}."),
            Ordering::Greater => println!("No, the bug is in an equation before {guess}."),
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
