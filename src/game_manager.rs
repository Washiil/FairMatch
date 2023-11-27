use crate::player_manager::Player;

use uuid::Uuid;

pub struct Game {
	id: uuid::Uuid,
	team_1: Vec<Player>,
	team_2: Vec<Player>,
	pub fairness: i32
}

pub struct GameManager {
	pub total_games: i64
}

impl GameManager {
	pub fn new() -> Self {
		GameManager {
			total_games: 0
		}
	}

	pub fn create_game(&mut self, mut players: Vec<Player>) -> Game {
		let mut sorted_players: Vec<Player> = players.clone();
        sorted_players.sort_by(|a: &Player, b: &Player| b.skill.partial_cmp(&a.skill).unwrap_or(std::cmp::Ordering::Equal));

		let mut team1: Vec<Player> = Vec::new();
		let mut t1: i32 = 0;
        let mut team2: Vec<Player> = Vec::new();
		let mut t2: i32 = 0;

		for (i, player) in sorted_players.into_iter().enumerate() {
			if i % 2 == 0 {
					t1 += &player.skill;
					team1.push(player);
			} else {
					t2 += &player.skill;
					team2.push(player);
			}
		}

		t1 = t1 / team1.len() as i32;
		t2 = t2 / team2.len() as i32;

        self.total_games += 1;

		Game {
			id: Uuid::new_v4(),
			team_1: team1,
			team_2: team2,
			fairness: (t1 - t2).abs()
		}
	}
}