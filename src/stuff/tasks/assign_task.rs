use crate::stuff::tasks::fix_model::fix_model;
use crate::stuff::tasks::read_doc::read_doc;

use rand::random_range;

pub fn assign_task(tenure: i32) -> fn(i32) -> bool {
    // Make a task index that based on tenure and randomness
    let task_index = tenure + random_range(0..50);

    // Return task based on tenure
    if task_index <= 20 {
        println!("\nYou've been tasked with running the model.");
        fix_model
    } else {
        println!("\nYou've been tasked with reading some documentation.");
        read_doc
    }
}
