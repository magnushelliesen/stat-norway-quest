// Enum that holds job titles
pub enum JobTitle {
    HigherExecutiveOfficer,
    Adviser,
    SeniorAdviser,
    HeadOfDivision,
    DirectorOfDepartment,
}

impl JobTitle {
    fn as_string(&self) -> String {
        match &self {
            JobTitle::HigherExecutiveOfficer => String::from("Higher Executive Officer"),
            JobTitle::Adviser => String::from("Adviser"),
            JobTitle::SeniorAdviser => String::from("Senior Adviser"),
            JobTitle::HeadOfDivision => String::from("SHead of Division"),
            JobTitle::DirectorOfDepartment => String::from("Director of Department"),
        }
    }
}

// Struct that holds player stats
pub struct Stats {
    pub name: String,
    pub tenure: i32,
    pub skill: i32,
    pub respect: i32,
    pub amount_written: i32,
    pub job_title: JobTitle,
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
    // Method that prints stats
    pub fn print_stats(self) {
        let name: String = self.name;
        let tenure: i32 = self.tenure;
        let skill: i32 = self.skill;
        let respect: i32 = self.respect;
        let amount_written: i32 = self.amount_written;
        let job_title: String = self.job_title.as_string();

        println!(
            "Name: {name} \n\
            Tenure: {tenure} \n\
            Skill: {skill} \n\
            Respect: {respect} \n\
            Amount written: {amount_written} \n\
            Job title: {job_title}"
        )
    }

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
