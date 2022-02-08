use clap::ArgMatches;

pub struct Note {
    pub id: usize,
    pub text: String,
}

impl Note {
    pub fn get_id(sub_m: &ArgMatches) -> Result<usize, Box<dyn std::error::Error>> {
        Ok(sub_m
            .value_of("id")
            .expect("please enter id.")
            .parse::<usize>()?)
    }
}

impl Clone for Note {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            text: self.text.clone(),
        }
    }
}
