use rand::random_range;

pub fn assign_task(tenure: i32) -> fn() -> bool{
    let random_task = random_range(1..100);

    // Return task based on tenure
    if tenure <= random_task {
        fix_model
    } else {
        fix_model
    }
}

pub fn fix_model() -> bool {
    true
}