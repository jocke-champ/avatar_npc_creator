use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::prelude::*;

pub fn expand_training(training: &str, importance: &str) -> String {
    let path = match training {
        "air bending" => "./data/techniques/air_bending_techniques.txt",
        "water bending" => "./data/techniques/water_bending_techniques.txt",
        "earth bending" => "./data/techniques/earth_bending_techniques.txt",
        "fire bending" => "./data/techniques/fire_bending_techniques.txt",
        "technology" => "./data/techniques/technology_techniques.txt",
        "weapons" => "./data/techniques/weapon_techniques.txt",
        _ => panic!("What training is this? O_o"),
    };

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut line_count = 0;
    let stop_count = match importance {
        "minor" => 1,
        "major" => 2,
        "master" | "legendary" => 3,
        _ => panic!("What is this? O_o"),
    };

    for line in reader.lines() {
        line_count += 1;
        let l = line;
        if line_count == stop_count {
            return format!("{}: {}",training, l.unwrap());
        }
    }
    "".to_string()
}

pub fn read_in_stat(file_path: Option<&str>, nationality: Option<&String>) -> String {
    let filepath;
    let nat = if nationality != None {
        nationality.unwrap().as_str()
    } else {
        ""
    };

    if file_path == None {
        match nat {
            "water" => filepath = "./data/water_names.txt",
            "air" => filepath = "./data/air_names.txt",
            "earth" => filepath = "./data/earth_names.txt",
            "fire" => filepath = "./data/fire_names.txt",
            _ => panic!("NO FILE PATH OR NATIONALITY PROVIDED"),
        }
    } else {
        filepath = file_path.unwrap();
    }

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut stats: Vec<String> = vec![];

    for line in reader.lines() {
        stats.push(line.unwrap());
    }

    if file_path.is_some() && nationality.is_some() {
        stats.retain(|s| !s.contains("bending") || s.contains(nat));
    }

    stats[(rand::random::<usize>() % stats.len())].to_string()
}

pub fn generate_principle(importance: &str) -> u8 {
    match importance {
        "minor" => thread_rng().gen_range(0..=1),
        "major" => thread_rng().gen_range(0..=2),
        "master" => thread_rng().gen_range(0..=3),
        "legendary" => thread_rng().gen_range(0..=4),
        _ => panic!("please specify the importance value of the character (minor, major, master, legendary)"),
    }
}