use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::{prelude::*, Rng, seq::SliceRandom};


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
}

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
}

pub fn expand_training(training: Training, importance: &Importance) -> String {
    let path = training.get_path();
    let stop_count = importance.get_stop_count();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut line_count = 0;
    for line in reader.lines() {
        line_count += 1;
        if line_count == stop_count {
            return format!("{}: {}",training.display(), line.unwrap())
        }
    }
    "".to_string()
}

pub fn read_in_stat(file_path: &str, nationality: &str) -> String {
    let filepath = match (file_path, nationality) {
        (path, "") => path,
        ("", nat) => match nat {
            "water" => "./data/names/water_names.txt",
            "air" => "./data/names/air_names.txt",
            "earth" => "./data/names/earth_names.txt",
            "fire" => "./data/names/fire_names.txt",
            _ => panic!("NO FILE PATH OR NATIONALITY PROVIDED"),
        },
        (_path, _nat) => {
            panic!("Both file_path and nationality are provided, please provide only one of them")
        }
    };

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut stats: Vec<String> = vec![];

    for line in reader.lines() {
        stats.push(line.unwrap());
    }

    if nationality != "" {
        stats.retain(|s| !s.contains("bending") || s.contains(nationality));
    }

    stats[(rand::random::<usize>() % stats.len())].to_string()
}

pub fn generate_principle(importance: &String) -> u8 {
    let mut rng = rand::thread_rng();
    let max_principle = match importance.as_str() {
        "minor" => 1,
        "major" => 2,
        "master" => 3,
        "legendary" => 4,
        _ => panic!("please specify the importance value of the character (minor, major, master, legendary)"),
    };
    rng.gen_range(0..=max_principle + 1)
}

pub fn generate_age() -> u8 {
    let mut rng = thread_rng();
    rng.gen_range(3..=100)
}

pub fn get_file_path(input: &str) -> String {
    format!("./data/{}.txt", input)
}


