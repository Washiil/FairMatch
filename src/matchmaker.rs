use std::collections::{HashMap, VecDeque};

use rand::{rngs::StdRng, SeedableRng, Rng};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Player {
    pub id: uuid::Uuid,
    pub skill: i32,
}

pub struct Game {
	id: uuid::Uuid,
	team_1: Vec<Player>,
	team_2: Vec<Player>,
	pub fairness: f64,
}

pub struct MatchMaker {
	pub total_players: i32,
	pub total_games: i32,
	rng: StdRng,
	pub player_queue: VecDeque<Player>,
	pub avg_fairness: f64,
}

impl MatchMaker {
	pub fn new(seed: u64) -> MatchMaker {
		MatchMaker { 
			total_players: 0, 
			total_games: 0, 
			player_queue: VecDeque::new(),
			avg_fairness: 0.0,
			rng: StdRng::seed_from_u64(seed),
		}
	}

	fn rolling_avg(&mut self, value: f64) {
		// Update mean using Welford's algorithm
		self.avg_fairness += (value - self.avg_fairness) / self.total_games as f64;
	}

	pub fn generate_player(&mut self) {
		let p = Player {
				id: Uuid::new_v4(),
				skill: self.rng.gen_range(0..1000),
		};
		self.player_queue.push_back(p);
		self.total_players += 1;
	}

	pub fn generate_game(&mut self, players: Vec<Player>) -> Game {
		let mut sorted_players: Vec<Player> = players.clone();
		sorted_players.sort_by(|a: &Player, b: &Player| {
				b.skill
				.partial_cmp(&a.skill)
				.unwrap_or(std::cmp::Ordering::Equal)
		});

		let mut team1: Vec<Player> = Vec::new();
		let mut team_1_skill: i32 = 0;
		let mut team2: Vec<Player> = Vec::new();
		let mut team_2_skill: i32 = 0;

		for (i, player) in sorted_players.into_iter().enumerate() {
				if i % 2 == 0 {
					team_1_skill += &player.skill;
						team1.push(player);
				} else {
					team_2_skill += &player.skill;
						team2.push(player);
				}
		}

		let t1: f64 = team_1_skill as f64 / team1.len() as f64;
		let t2 = team_2_skill as f64 / team2.len() as f64;
		let fairness = (t1 - t2).abs();

		self.total_games += 1;
		self.rolling_avg(fairness);

		return Game {
				id: Uuid::new_v4(),
				team_1: team1,
				team_2: team2,
				fairness: fairness,
		}
	}
}