use std::fs::File;
use std::io::{BufRead, BufReader};
use text_io::scan;
use rand::prelude::*;

#[allow(dead_code)]
#[derive(Debug)]
struct NPC {
    name: String,
    training: String,
    drive: String,
    principle: u8,
}

fn generate_npc(nationality: String, importance: String) -> NPC {

    let name = read_in_stat(None, Some(&nationality));

    let mut training = read_in_stat(Some("./data/training.txt"), Some(&nationality));

    training = expand_training(training.as_str(), importance.to_ascii_lowercase().as_str());

    let drive = read_in_stat(Some("./data/drives.txt"), None);

    let principle: u8 = match importance.to_ascii_lowercase().as_str() {
        "minor" => thread_rng().gen_range(0..=1),
        "major" => thread_rng().gen_range(0..=2),
        "master" => thread_rng().gen_range(0..=3),
        "legendary" => thread_rng().gen_range(0..=4),
        _ => panic!("please specify the importance value of the character (minor, major, master, legendary)"),
    };

    NPC {
        name,
        training,
        drive,
        principle,
    }
}

fn read_in_stat(file_path: Option<&str>, nationality: Option<&String>) -> String {
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

    if file_path != None && nationality != None {
        stats.retain(|s| !s.contains("bending") || s.contains(nat));
    }

    stats[(rand::random::<usize>() % stats.len())].to_string()
}

fn expand_training(training: &str, importance: &str) -> String {
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

fn main() {
    let (nationality, importance);
    println!("Enter Nationality (Water, Earth, Fire, Air) Followed By Importance (Minor, Major, Master, Legendary) :");
    scan!("{} {}", nationality, importance);

    let npc = generate_npc(nationality, importance);
    println!("\n❃❃❃❃❃❃❃❃❃❃");
    println!("{}", npc.name);
    println!("{}\ndrive: {}\nprinciple: {}", npc.training, npc.drive, npc.principle);
}