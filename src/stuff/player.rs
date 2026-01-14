pub struct Stats {
    pub name: String,
    pub tenure: i32,
    pub skill: i32,
    pub respect: i32,
    pub amount_written: i32,
    pub job_title: String,
}

pub fn make_player(name: String) -> Stats {
    Stats {
        name,
        tenure: 0,
        skill: 0,
        respect: 0,
        amount_written: 0,
        job_title: String::from("Higher executive officer"),
    }
}

impl Stats {
    pub fn increment_tenure(&mut self){
        self.tenure +=1 ;
    }
}