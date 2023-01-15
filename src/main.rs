use text_io::scan;
mod funcs;

#[allow(dead_code)]
#[derive(Debug)]
struct NPC {
    name: String,
    training: String,
    drive: String,
    principle: u8,
}

fn generate_npc(nationality: String, importance: String) -> NPC {

    let name = funcs::read_in_stat(None, Some(&nationality));

    let mut training = funcs::read_in_stat(Some("./data/training.txt"), Some(&nationality));

    training = funcs::expand_training(training.as_str(), importance.to_ascii_lowercase().as_str());

    let drive = funcs::read_in_stat(Some("./data/drives.txt"), None);

    let principle: u8 = funcs::generate_principle(importance.to_ascii_lowercase().as_str());

    NPC {
        name,
        training,
        drive,
        principle,
    }
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