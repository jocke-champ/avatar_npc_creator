use serde_json::{from_reader};
use std::fs::File;

use crate::funcs::NPC;

pub fn read_npc_json() -> Result<Vec<NPC>, serde_json::Error> {
    let file = File::open("npc.json").unwrap();
    let npcs: Vec<NPC> = from_reader(file)?;
    Ok(npcs)
}