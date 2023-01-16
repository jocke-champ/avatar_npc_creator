use funcs::generate_age;
use text_io::scan;
mod funcs;

#[allow(dead_code)]
#[derive(Debug)]
struct NPC {
    name: String,
    training: String,
    drive: String,
    personality: String,
    occupation: String,
    age: u8,
    principle_display: String,
}

fn generate_npc(nationality: String, importance: String) -> Result<NPC, &'static str> {
    let training = match funcs::Training::random_from_str_lowercase(nationality.clone()) {
        Some(t) => match funcs::Importance::from_str_lowercase(&importance) {
            Some(i) => funcs::expand_training(t, &i),
            None => return Err("Invalid importance provided"),
        },
        None => return Err("Invalid nationality provided"),
    };
    let name = funcs::read_in_stat("", &nationality);
    let drive = funcs::read_in_stat(&funcs::get_file_path("drives"), "");
    let personality = funcs::read_in_stat(&funcs::get_file_path("personalities"), "");
    let occupation = funcs::read_in_stat(&funcs::get_file_path("occupations"), "");
    let principle = match funcs::Importance::from_str_lowercase(&importance) {
        Some(_) => funcs::generate_principle(&importance),
        None => return Err("Invalid importance provided"),
    };
    let age = generate_age();
    let principle_display = funcs::Importance::from_str_lowercase(&importance).unwrap().principle_string(principle);
    // let occupation

    Ok(NPC {
        name,
        training,
        drive,
        personality,
        occupation,
        age,
        principle_display,
    })
}

fn main() {
    let (nationality, importance);
    println!("Enter Nationality (Water, Earth, Fire, Air) Followed By Importance (Minor, Major, Master, Legendary) :");
    scan!("{} {}", nationality, importance);

    let npc = generate_npc(nationality, importance).unwrap_or_else(|e| {
        println!("Error: {}", e);
        std::process::exit(1);
    });
    println!("\n❃❃❃❃❃❃❃❃❃❃\n");
    println!("Name: {}", npc.name);
    println!("Training: {}", npc.training);
    println!("Drive: {}", npc.drive);
    println!("Personality: {}", npc.personality);
    println!("Occupation: {}", npc.occupation);
    println!("Principle: {}", npc.principle_display);
    println!("Age: {}", npc.age);
}