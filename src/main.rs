mod player;

use crate::player::make_player;
use std::io;

fn main() {
    // Accept player name before game starts
    let mut player_name = String::new();

    println!("Welcome to Stat Norway Quest, please type your name:");

    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    // Instantiate player struct
    let _player = make_player(player_name);
}
