use crate::stuff::player::make_player;
use crate::stuff::tasks::assign_task;
use std::io;

pub mod stuff;

fn main() {
    // Accept player name before game starts
    let mut player_name = String::new();

    println!("Welcome to Stat Norway Quest, please type your name:");

    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    // Instantiate Stats struct
    let mut stats = make_player(player_name);

    // Game loop
    loop {
        let task = assign_task(stats.tenure);

        if task() {
            println!("You successfully completed your task.");
            stats.increment_skill();
            stats.increment_respect();
        } else {
            println!("You unsuccessfully failed at your task.");
            stats.decrement_respect();
        }

        stats.increment_tenure();

        println!("{:#?}", stats);
        break;
    }

    println!("Congratulations! You just worked many many years in Statistics Norway.")
}
