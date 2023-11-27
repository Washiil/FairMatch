use uuid::Uuid;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[derive(Clone)]
pub struct Player {
    pub id: uuid::Uuid,
    pub skill: i32,
}

pub struct PlayerGenerator {
	total_players: i64,
	seed: StdRng
}

impl PlayerGenerator {
    pub fn new(seed: u64) -> Self {
        // Initialize any necessary state
        PlayerGenerator {
					total_players: 0,
					seed: StdRng::seed_from_u64(seed)
				}
    }

    pub fn generate_player(&mut self) -> Player {
				self.total_players += 1;
        return Player {
            id: Uuid::new_v4(),
            skill: self.seed.gen_range(0..1000),
        }
    }
}
