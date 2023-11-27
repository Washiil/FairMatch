use std::collections::VecDeque;

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Player {
    pub id: uuid::Uuid,
    pub skill: i32,
}

#[derive(Debug)]
pub struct PlayerManager {
    pub total_players: i64,
    pub player_queue: VecDeque<Player>,
    seed: StdRng,
}

impl PlayerManager {
    pub fn new(seed: u64) -> Self {
        // Initialize any necessary state
        PlayerManager {
            total_players: 0,
            player_queue: VecDeque::new(),
            seed: StdRng::seed_from_u64(seed),
        }
    }

    pub fn generate_player(&mut self) {
        let p = Player {
            id: Uuid::new_v4(),
            skill: self.seed.gen_range(0..1000),
        };
        self.player_queue.push_back(p);
        self.total_players += 1;
    }
}
