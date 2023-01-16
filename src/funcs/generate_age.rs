use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub fn generate_age() -> u8 {
    let mut rng = thread_rng();

    // Weights for age range 0-14, 15-69, 70-100
    let weights = vec![1, 15, 1];
    let age_dist = WeightedIndex::new(weights).unwrap();

    // Generate a random index within the weight range
    let age_range = age_dist.sample(&mut rng);

    // Map the index to the corresponding age range
    let age = match age_range {
        0 => rng.gen_range(0..15),
        1 => rng.gen_range(15..70),
        2 => rng.gen_range(70..101),
        _ => unreachable!()
    };

    age
}