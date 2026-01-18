use crate::stuff::player::make_player;
use crate::stuff::tasks::assign_task::assign_task;
use rand::random_bool;
use std::io;

pub mod stuff;

// Constants (might be put in some separate file later)
const PROBABILITY_SALARY_NEGOTIATION: f64 = 0.05;
const PROBABILITY_XMAS_PARTY: f64 = 0.05;

fn main() {
    // Accept player name before game loop starts
    let mut player_name = String::new();

    println!("\n\nWelcome to STAT NORWAY QUEST I, \nplease type your name:");

    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    // Instantiate Stats struct using constructor
    let mut stats = make_player(player_name);

    println!(
        "\nCongratulations, you've been hired as an {}. Good luck and Godspeed.\n",
        stats.job_title.as_string()
    );

    // Game loop
    loop {
        println!("\nWelcome to day {}.\n", stats.tenure + 1);
        /*
        Every day the following happens
        1) The player gets a task
        2) The player has to interact with some co-worker
        4) With some probability, the player negotiates salary
        5) With some probability, it's the office Xmas party
        */

        // 1) Random task

        // Assign a random task given tenure
        let task = assign_task(stats.tenure);

        // Determine if task was completed successfully
        if task(stats.tenure) {
            println!("You successfully completed your task.");
            stats.increment_skill();
            stats.increment_respect();
        } else {
            println!("You unsuccessfully failed at your task.");
            stats.decrement_respect();
        }

        stats.increment_tenure();

        // 2) Interact with some co-worker TBA

        // 3) Salary negotiation
        if random_bool(PROBABILITY_SALARY_NEGOTIATION) {
            println!("Salary negotiation.")
        }

        // 4) Xmas party
        if random_bool(PROBABILITY_XMAS_PARTY) {
            println!("Xmas party.")
        }

        //println!("{:#?}", stats);

        stats.print_stats();

        break;
    }

    println!("Congratulations! You just worked many many years in Statistics Norway.")
}
