use rand::{Rng, SeedableRng, rngs::StdRng};

pub struct SeedRng {
    pub seed: u64,
    pub rng: StdRng,
}

impl SeedRng {
    pub fn new(seed: Option<u64>) -> Self {
        let actual_seed = seed.unwrap_or_else(|| rand::rng().random());

        SeedRng {
            seed: actual_seed,
            rng: StdRng::seed_from_u64(actual_seed),
        }
    }
}
