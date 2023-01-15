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

    let name = funcs::read_in_stat("", &nationality);

    let training = funcs::expand_training(funcs::Training::random_from_str_lowercase(nationality).unwrap(), &funcs::Importance::from_str_lowercase(&importance).unwrap());

    let drive = funcs::read_in_stat("./data/drives.txt", "");

    let principle: u8 = funcs::generate_principle(importance);

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