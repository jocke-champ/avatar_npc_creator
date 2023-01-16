use rand::thread_rng;
use rand::prelude::*;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use crate::funcs::importance::Importance;

#[derive(Debug)]
#[derive(Clone)]
pub enum Training {
    AirBending,
    WaterBending,
    EarthBending,
    FireBending,
    Technology,
    Weapons,
}

impl Training {
    fn get_path(&self) -> &str {
        match self {
            Training::AirBending => "./data/techniques/air_bending_techniques.txt",
            Training::WaterBending => "./data/techniques/water_bending_techniques.txt",
            Training::EarthBending => "./data/techniques/earth_bending_techniques.txt",
            Training::FireBending => "./data/techniques/fire_bending_techniques.txt",
            Training::Technology => "./data/techniques/technology_techniques.txt",
            Training::Weapons => "./data/techniques/weapon_techniques.txt",
        }
    }
    pub fn from_str_lowercase(s: String) -> Option<Self> {
        match s.as_str() {
            "air" => Some(Training::AirBending),
            "water" => Some(Training::WaterBending),
            "earth" => Some(Training::EarthBending),
            "fire" => Some(Training::FireBending),
            _ => None,
        }

    }

    pub fn random_from_str_lowercase(s: String) -> Option<Training> {
        let mut pool = vec![Training::Weapons, Training::Technology];
        if let Some(training) = Training::from_str_lowercase(s) {
            pool.push(training);
        }
        pool.choose(&mut thread_rng()).cloned()
    }
    pub fn display(&self) -> &str {
        match self{
            Training::AirBending => "Air Bending",
            Training::WaterBending => "Water Bending",
            Training::EarthBending => "Earth Bending",
            Training::FireBending => "Fire Bending",
            Training::Technology => "Technology",
            Training::Weapons => "Weapons",
        }
    }
    pub fn expand_training(&self, importance: &Importance) -> String {
        let path = self.get_path();
    let stop_count = importance.get_stop_count();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut line_count = 0;
    for line in reader.lines() {
        line_count += 1;
        if line_count == stop_count {
            return format!("{}: {}",self.display(), line.unwrap())
        }
    }
    "".to_string()
    }
}