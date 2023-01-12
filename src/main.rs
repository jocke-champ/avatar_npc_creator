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
    technique: String,
}

fn generate_npc(nationality: String, importance: String) -> NPC {

    // let nationality_file = match nationality.to_ascii_lowercase().as_str() {
    //     "water" => String::from("./data/water_names.txt"),
    //     "fire" => String::from("./data/fire_names.txt"),
    //     "earth" => String::from("./data/earth_names.txt"),
    //     "air" => String::from("./data/air_names.txt"),
    //     _ => panic!("please specify a nationality (water, fire, earth and air)"),
    // };

    let importance_value: u8 = match importance.to_ascii_lowercase().as_str() {
        "minor" => 1,
        "major" => thread_rng().gen_range(2..=3),
        "master" => thread_rng().gen_range(4..=5),
        "legendary" => thread_rng().gen_range(6..=8),
        _ => panic!("please specify the importance value of the character (minor, major, master, legendary)"),
    };

    //name
    // let mut file = File::open(nationality_file).unwrap();
    // let mut reader = BufReader::new(file);
    // let mut names: Vec<String> = vec![];
    // for line in reader.lines() {
    //     names.push(line.unwrap());
    // }
    // let name = names[(rand::random::<usize>() %
    //     names.len())].to_string();
    let name = read_in_stat(None, Some(&nationality));

    //training
    // file = File::open("./data/training.txt").unwrap();
    // reader = BufReader::new(file);
    // let mut trainings: Vec<String> = vec![];

    // for line in reader.lines() {
    //     trainings.push(line.unwrap());
    // }
    // let mut training = trainings[(rand::random::<usize>() %
    //     trainings.len())].to_string();
    let mut training = read_in_stat(Some("./data/training.txt"), Some(&nationality));


    match training.as_str() {
        "air bending" => match importance.to_ascii_lowercase().as_str() {
            "minor" => training = format!("{}: {}", training, "Big gust of wind"),
            "major" => training = format!("{}: {}", training, "Throwing incoming physical physical attacks off-course with wind, gliding along on air currents or balls of whirling air, air currents or balls of whirling air"),
            "master" | "legendary" => training = format!("{}: {}", training, "Impossible grace, near-weightlessness, perfect dogding, gale-force winds"),
            _ => panic!("How important is this guy? O_o"),
        },
        "water bending" => match importance.to_ascii_lowercase().as_str() {
            "minor" => training = format!("{}: {}", training, "Channeling big jets of water"),
            "major" => training = format!("{}: {}", training, "Use ice creatively to shape the environment, heal(rarer)"),
            "master" | "legendary" => training = format!("{}: {}", training, "Instantly changing water to ice and back, many limbs of water, sliding ice"),
            _ => panic!("Sus water boy"),
        },
        "fire bending" => match importance.to_ascii_lowercase().as_str() {
            "minor" => training = format!("{}: {}", training, "Throw gouts of flame"),
            "major" => training = format!("{}: {}", training, "Light the environment aflame, launch themselves into the air with fire"),
            "master" | "legendary" => training = format!("{}: {}", training, "Throwing fire at long distances, enormous fireballs, waves of flame"),
            _ => panic!("Fiery pants!"),
        },
        "earth bending" => match importance.to_ascii_lowercase().as_str() {
            "minor" => training = format!("{}: {}", training, "Hurl rocks"),
            "major" => training = format!("{}: {}", training, "Change the environment, exhibit seismic sense"),
            "master" | "legendary" => training = format!("{}: {}", training, "Instant shiftss from defense to offense (wall of earth turns into hurled rock), precision control, massive control"),
            _ => panic!("BOWLING!"),
        },
        "technology" => match importance.to_ascii_lowercase().as_str() {
            "minor" => training = format!("{}: {}", training, "Obviously dangerous machines (harpoon, guns, big tanks)"),
            "major" => training = format!("{}: {}", training, "Tricks, surprise weapons, and unexpected capabilities -- but only a few"),
            "master" | "legendary" => training = format!("{}: {}", training, "Endless supplies of small devices with dangerous uses"),
            _ => panic!("Screwed up!"),
        },
        "weapons" => match importance.to_ascii_lowercase().as_str() {
            "minor" => training = format!("{}: {}", training, "Swing with regular weapons"),
            "major" => training = format!("{}: {}", training, "Dance around acrobatically, attack with strength and power"),
            "master" | "legendary" => training = format!("{}: {}", training, "Distinctive and difficult weapons used with perfect precision and might"),
            _ => panic!("I gout cut :("),
        }
        _ => panic!("Training Gone Wrong! O_o"),
    }

    //drive
    // file = File::open("./data/drives.txt").unwrap();
    // reader = BufReader::new(file);
    // let mut drives: Vec<String> = vec![];

    // for line in reader.lines() {
    //     drives.push(line.unwrap());
    // }
    // let drive = drives[(rand::random::<usize>() %
    //     drives.len())].to_string();
    let drive = read_in_stat(Some("./data/drives.txt"), None);

    //principle
    let principle = thread_rng().gen_range(0..=importance_value);

    //technique
    // file = File::open("./data/techniques.txt").unwrap();
    // reader = BufReader::new(file);
    // let mut techniques: Vec<String> = vec![];

    // for line in reader.lines() {
    //     techniques.push(line.unwrap());
    // }
    // let technique = techniques[(rand::random::<usize>() %
    //     techniques.len())].to_string();
    let technique = read_in_stat(Some("./data/techniques.txt"), None);

    NPC {
        name,
        training,
        drive,
        principle,
        technique,
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

fn main() {
    let (nationality, importance);
    println!("Enter Nationality (Water, Earth, Fire, Air) Followed By Importance (Minor, Major, Master, Legendary) :");
    scan!("{} {}", nationality, importance);

    let npc = generate_npc(nationality, importance);
    println!("NPC: {:#?}", npc);
}