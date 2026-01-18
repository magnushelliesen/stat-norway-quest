use crate::stuff::tasks::fix_model_task::fix_model;

use rand::random_range;

pub fn assign_task(tenure: i32) -> fn(i32) -> bool {
    // Make a task index that based on tenure and randomness
    let task_index = tenure + random_range(1..100);

    // Return task based on tenure
    if task_index <= 1 {
        println!("\nYou've been tasked with running the model.");
        fix_model
    } else {
        println!("\nYou've been tasked with running the model.");
        fix_model
    }
}
