use serde_json::{from_reader, to_writer};
use std::fs::File;
mod funcs;
use funcs::NPC;

fn main() {
    let (nationality, importance);
    println!("Enter Nationality (Water, Earth, Fire, Air) Followed By Importance (Minor, Major, Master, Legendary) :");
    text_io::scan!("{} {}", nationality, importance);

    let new_npc = NPC::generate_npc(nationality, importance).unwrap();

    // Open the npc.json file for reading
    let file = match File::open("npc.json") {
        Ok(file) => file,
        Err(_) => {
            let file = File::create("npc.json").unwrap();
            file
        }
    };

    let mut npcs: Vec<NPC> = match from_reader(file) {
        Ok(npcs) => npcs,
        Err(_) => vec![],
    };

    // Push the new NPC to the vector of NPCs
    npcs.push(new_npc);

    // Open the npc.json file for writing
    let file = File::create("npc.json").unwrap();

    // Serialize the updated vector of NPCs and write it to the npc.json file
    to_writer(file, &npcs).unwrap();
}