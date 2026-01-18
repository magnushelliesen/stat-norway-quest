use rand::distr::{Alphanumeric, SampleString};

use std::io;
const NUMBER_OF_LETTERS: usize = 100;

pub fn read_doc(_: i32) -> bool {
    let string: String = Alphanumeric.sample_string(&mut rand::rng(), NUMBER_OF_LETTERS);

    println!(
        "\nRead the following documentation:\n\
        {string}\n\
        \nTo verify that you read it, type in evey other letter (one by one):"
    );

    let mut success: bool = true;

    for (i, x) in string.chars().enumerate() {
        if i % 2 == 0 {
            let mut player_response: String = String::new();

            io::stdin()
                .read_line(&mut player_response)
                .expect("Failed to read line");

            if player_response.trim().to_string() != x.to_string() {
                println!(
                    "Woopsie.. You wrote {}, but the correct response was {}. Bummer...",
                    player_response.trim().to_string(),
                    x.to_string()
                );
                success = false;
                break;
            }
        }
    }

    success
}
