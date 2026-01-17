pub enum JobTitle {
    HigherExecutiveOfficer,
    Advisor,
    SeniorAdvisor,
}

pub struct Stats {
    pub name: String,
    pub tenure: i32,
    pub skill: i32,
    pub respect: i32,
    pub amount_written: i32,
    pub job_title: JobTitle,
}

pub fn make_player(name: String) -> Stats {
    Stats {
        name,
        tenure: 0,
        skill: 10,
        respect: 10,
        amount_written: 0,
        job_title: JobTitle::HigherExecutiveOfficer,
    }
}

impl Stats {
    pub fn increment_tenure(&mut self) {
        self.tenure += 1;
    }

    pub fn increment_skill(&mut self) {
        self.respect += 1;
    }

    pub fn increment_respect(&mut self) {
        self.respect += 1;
    }

    pub fn decrement_respect(&mut self) {
        self.respect -= 1;
    }

    pub fn implement_promotion(&mut self) {
        self.job_title = match self.job_title {
            JobTitle::HigherExecutiveOfficer => JobTitle::Advisor,
            JobTitle::Advisor => JobTitle::SeniorAdvisor,
            _ => JobTitle::Advisor,
        }
    }

    pub fn implement_deomotion(&mut self) {
        self.job_title = match self.job_title {
            JobTitle::SeniorAdvisor => JobTitle::Advisor,
            JobTitle::Advisor => JobTitle::HigherExecutiveOfficer,
            _ => JobTitle::HigherExecutiveOfficer,
        }
    }
}
