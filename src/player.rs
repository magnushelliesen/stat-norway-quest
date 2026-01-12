pub struct Player {
    pub name: String,
    pub skill: i32,
    pub respect: i32,
    pub amount_written: i32,
    pub job_title: String,
}

pub fn make_player(name: String) -> Player {
    Player {
        name,
        skill: 0,
        respect: 0,
        amount_written: 0,
        job_title: String::from("Higher executive officer"),
    }
}
