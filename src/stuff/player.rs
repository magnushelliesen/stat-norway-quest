// Struct that holds player stats
#[derive(Debug)]
pub struct Stats {
    pub name: String,
    pub tenure: i32,
    pub skill: i32,
    pub respect: i32,
    pub amount_written: i32,
    pub job_title: JobTitle,
}

// Enum that holds job titles
#[derive(Debug)]
pub enum JobTitle {
    HigherExecutiveOfficer,
    Adviser,
    SeniorAdviser,
    HeadOfDivision,
    DirectorOfDepartmen,
}

// Function that generates player stats
pub fn make_player(name: String) -> Stats {
    Stats {
        name: name.trim().to_string(),
        tenure: 0,
        skill: 10,
        respect: 10,
        amount_written: 0,
        job_title: JobTitle::HigherExecutiveOfficer,
    }
}

impl Stats {
    /*
    Methods that increment tenure and skill (these can only increase),
    and increment and decrement respect (these can increase and decrease)
     */
    pub fn increment_tenure(&mut self) {
        self.tenure += 1;
    }

    pub fn increment_skill(&mut self) {
        self.skill += 1;
    }

    pub fn increment_respect(&mut self) {
        self.respect += 1;
    }

    pub fn decrement_respect(&mut self) {
        self.respect -= 1;
    }

    // Methods that handle promotions and demotions
    pub fn implement_promotion(&mut self) {
        self.job_title = match self.job_title {
            JobTitle::HigherExecutiveOfficer => JobTitle::Adviser,
            JobTitle::Adviser => JobTitle::SeniorAdviser,
            _ => JobTitle::Adviser,
        }
    }

    pub fn implement_deomotion(&mut self) {
        self.job_title = match self.job_title {
            JobTitle::SeniorAdviser => JobTitle::Adviser,
            JobTitle::Adviser => JobTitle::HigherExecutiveOfficer,
            _ => JobTitle::HigherExecutiveOfficer,
        }
    }
}
