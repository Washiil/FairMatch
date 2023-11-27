mod player_generator;
mod game_manager;

use std::collections::VecDeque;
use std::io::Write;
use std::ops::Div;
use std::sync::{Arc, Mutex};
use std::thread;

use player_generator::Player;
use player_generator::PlayerGenerator;

use game_manager::Game;
use game_manager::GameManager;

use uuid::Uuid;
use rand::Rng;

fn main() {
    let total_players: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let total_players_clone: Arc<Mutex<i32>> = total_players.clone();

    let player_queue: Arc<Mutex<VecDeque<Player>>> = Arc::new(Mutex::new(VecDeque::new()));
    let player_queue_clone: Arc<Mutex<VecDeque<Player>>> = player_queue.clone();

    let total_games: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let total_games_clone: Arc<Mutex<i32>> = total_games.clone();

    let total_fairness: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let total_fairness_clone: Arc<Mutex<i32>> = total_fairness.clone();

    // Constantly generates players and adds them to the Deque
    thread::spawn(move || {
        let seed = 64;
        let mut player_generator = PlayerGenerator::new(seed);
        loop {
            let player = player_generator.generate_player();

            *total_players_clone.lock().unwrap() += 1;

            player_queue_clone.lock().unwrap().push_back(player);
        }
    });

    // Checks if we have enough players and creates new "fair" games
    thread::spawn(move || {
        let game_generator = GameManager::new();
        loop {
            let mut queue = player_queue.lock().unwrap();

            if queue.len() >= 10 {
                let players: Vec<Player> = queue.drain(..10).collect();
                drop(queue);
                let game = game_generator.create_game(players);

                *total_games_clone.lock().unwrap() += 1;
                *total_fairness_clone.lock().unwrap() += game.fairness;
            }

            // Add some delay or other synchronization mechanism
            // thread::sleep(std::time::Duration::from_millis(20));
        }
    });

    // Dianostics Thread
    loop {
        print!("\x1B[2J\x1B[1;1H");
        let tp = total_players.lock().unwrap().clone() as f64;
        let tg = total_games.lock().unwrap().clone() as f64;
        let tf = total_fairness.lock().unwrap().clone() as f64;
        println!("Players: {}", tp);
        println!("Total Games: {}", tg);
        println!("Average Fairness: {}", (tf / tg));
        thread::sleep(std::time::Duration::from_millis(100));
    }
}
