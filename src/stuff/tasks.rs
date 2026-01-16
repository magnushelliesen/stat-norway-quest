use rand::random_range;

pub fn assign_task(tenure: i32) -> fn() -> bool {
    let task_index = tenure + random_range(1..100);

    // Return task based on tenure
    if task_index <= 1 {
        fix_model
    } else {
        fix_model
    }
}

pub fn fix_model() -> bool {
    true
}