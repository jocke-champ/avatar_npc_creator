use serde_json::{from_reader, to_writer};
use std::fs::File;
use funcs::NPC;

// Define a struct for representing a relationship between two NPCs
#[derive(Serialize, Deserialize)]
struct Relationship {
    npc1: String,
    npc2: String,
    relationship: String,
}

fn generate_relationship() {
    // Deserialize the NPCs from the npc.json file
    let file = File::open("npc.json").unwrap();
    let npcs: Vec<NPC> = from_reader(file).unwrap();

    // Create a vector to hold the relationships
    let mut relationships: Vec<Relationship> = vec![];

    // Iterate through the NPCs and create relationships between them
    for i in 0..npcs.len() {
        for j in i+1..npcs.len() {
            let relationship = Relationship {
                npc1: npcs[i].name.clone(),
                npc2: npcs[j].name.clone(),
                relationship: "Friendship".to_string(),
            };
            relationships.push(relationship);
        }
    }

    // Open the relationships.json file for writing
    let file = File::create("relationships.json").unwrap();

    // Serialize the relationships and write them to the relationships.json file
    to_writer(file, &relationships).unwrap();
}
