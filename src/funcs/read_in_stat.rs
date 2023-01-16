use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;

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

    let mut rng = rand::thread_rng();
    stats[rng.gen_range(0..stats.len())].to_string()
}