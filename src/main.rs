mod player_manager;
mod game_manager;

use std::collections::VecDeque;
use std::io::Write;
use std::ops::Div;
use std::sync::{Arc, Mutex};
use std::thread;

use player_manager::Player;
use player_manager::PlayerManager;

use game_manager::Game;
use game_manager::GameManager;

use uuid::Uuid;
use rand::Rng;

fn main() {
    let gm: Arc<Mutex<GameManager>> = Arc::new(Mutex::new(GameManager::new()));
    let gm_clone: Arc<Mutex<GameManager>> = gm.clone();

    let pm: Arc<Mutex<PlayerManager>> = Arc::new(Mutex::new(PlayerManager::new(64)));
    let pm_gen_clone: Arc<Mutex<PlayerManager>> = pm.clone();
    let pm_game_clone: Arc<Mutex<PlayerManager>> = pm.clone();

    let total_fairness: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let total_fairness_clone: Arc<Mutex<i32>> = total_fairness.clone();

    // Constantly generates players and adds them to the Deque
    thread::spawn(move || {
        loop {
            pm_gen_clone.lock().unwrap().generate_player();
        }
    });

    // Checks if we have enough players and creates new "fair" games
    thread::spawn(move || {
        loop {
            let mut queue = pm_game_clone.lock().unwrap();

            while queue.player_queue.len() >= 10 {
                let players: Vec<Player> = queue.player_queue.drain(..10).collect();
                let game = gm_clone.lock().unwrap().create_game(players);

                *total_fairness_clone.lock().unwrap() += game.fairness;
            }
            drop(queue);
        }
    });

    // Dianostics Thread
    loop {
        thread::sleep(std::time::Duration::from_millis(1000));
        print!("\x1B[2J\x1B[1;1H");
        let tp = pm.lock().unwrap().total_players as f64;
        let tg = gm.lock().unwrap().total_games as f64;
        let tf = total_fairness.lock().unwrap().clone() as f64;
        println!("Players: {}", tp);
        println!("Total Games: {}", tg);
        println!("Total Fairness: {}", tf);
        println!("Average Fairness: {}", (tf / tg));
        thread::sleep(std::time::Duration::from_millis(1000));
    }
}
