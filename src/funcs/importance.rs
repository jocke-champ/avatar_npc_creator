use rand::prelude::*;

pub enum Importance {
    Minor,
    Major,
    Master,
    Legendary,
}

impl Importance {
    pub fn get_stop_count(&self) -> usize {
        match self {
            Importance::Minor => 1,
            Importance::Major => 2,
            Importance::Master => 3,
            Importance::Legendary => 3,
        }
    }
    pub fn from_str_lowercase(s: &str) -> Option<Self> {
        match s {
            "minor" => Some(Importance::Minor),
            "major" => Some(Importance::Major),
            "master" => Some(Importance::Master),
            "legendary" => Some(Importance::Legendary),
            _ => None,
        }
    }
    pub fn principle_string(&self, assigned: u8) -> String {
        let max_principle = match self {
            Importance::Minor => 1,
            Importance::Major => 2,
            Importance::Master => 3,
            Importance::Legendary => 4,
        };
        format!("{}{}", "[*]".repeat(assigned as usize), "[]".repeat(max_principle as usize - assigned as usize))
    }
    pub fn generate_principle(&self) -> u8 {
        let max_principle = match self {
            Importance::Minor => 1,
            Importance::Major => 2,
            Importance::Master => 3,
            Importance::Legendary => 4,
        };
        thread_rng().gen_range(0..=max_principle)
    }
}