use crate::funcs::generate_age;
use crate::funcs::Training;
use crate::funcs::get_file_path;
use crate::funcs::read_in_stat;
use crate::funcs::Importance;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NPC {
    name: String,
    training: String,
    drive: String,
    personality: String,
    occupation: String,
    belief: String,
    quirk: String,
    physical_traits: String,
    age: u8,
    principle_display: String,
}

impl NPC {
    pub fn generate_npc(nationality: String, importance: String) -> Result<NPC, &'static str> {
        let training = match Training::random_from_str_lowercase(nationality.clone()) {
            Some(t) => match Importance::from_str_lowercase(&importance) {
                Some(i) => t.expand_training(&i),
                None => return Err("Invalid importance provided"),
            },
            None => return Err("Invalid nationality provided"),
        };
        let name = read_in_stat("", &nationality);
        let drive = read_in_stat(&get_file_path("personal/drives"), "");
        let personality = read_in_stat(&get_file_path("personal/personalities"), "");
        let occupation = read_in_stat(&get_file_path("personal/occupations"), "");
        let belief = read_in_stat(&get_file_path("personal/personal_believes"), "");
        let quirk = read_in_stat(&get_file_path("personal/quirks"), "");
        let physical_traits = read_in_stat(&get_file_path("personal/physical_traits"), "");
        let principle = match Importance::from_str_lowercase(&importance) {
            Some(i) => i.generate_principle(),
            None => return Err("Invalid importance provided"),
        };
        let age = generate_age();
        let principle_display = Importance::from_str_lowercase(&importance).unwrap().principle_string(principle);
        Ok(NPC {
            name,
            training,
            drive,
            personality,
            occupation,
            belief,
            quirk,
            physical_traits,
            age,
            principle_display,
        })
    }
}