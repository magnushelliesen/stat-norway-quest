mod player;

use crate::player::Player;
use std::io;

fn main() {
    // Accept player name before game starts
    let mut player_name = String::new();

    println!("Welcome to Stat Norway Quest, please type your name:");

    io::stdin()
            .read_line(&mut player_name)
            .expect("Failed to read line");

    // Instantiate player struct
    let _player = Player {
        name: String::from(player_name),
        skill: 0,
        respect: 0,
        amount_written: 0,
        job_title: String::from("Førstekonsulent")
    };
}
