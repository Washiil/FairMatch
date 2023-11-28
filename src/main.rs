mod player_manager;
mod game_manager;
mod matchmaker;

use std::collections::VecDeque;
use std::io::Write;
use std::ops::Div;
use std::sync::{Arc, Mutex};
use std::thread;

use matchmaker::MatchMaker;
use matchmaker::Game;
use matchmaker::Player;
use player_manager::PlayerManager;

use game_manager::GameManager;

use uuid::Uuid;
use rand::Rng;

fn main() {
    let seed = 64;
    let manager: Arc<Mutex<MatchMaker>> = Arc::new(Mutex::new(MatchMaker::new(seed)));
    let manager_clone = manager.clone();
    let pm_clone = manager.clone();

    // Constantly generates players and adds them to the Deque
    thread::spawn(move || {
        loop {
            pm_clone.lock().unwrap().generate_player();
        }
    });

    // Checks if we have enough players and creates new "fair" games
    thread::spawn(move || {
        loop {
            let mut handler = manager_clone.lock().unwrap();

            while handler.player_queue.len() >= 10 {
                let players: Vec<Player> = handler.player_queue.drain(..10).collect();
                let game = handler.generate_game(players);
            }
            drop(handler);
        }
    });

    // Dianostics Thread
    loop {
        print!("\x1B[2J\x1B[1;1H");

        let data = manager.lock().unwrap();
        let tp = data.total_players as f64;
        let tg = data.total_games as f64;
        let fairness = data.avg_fairness;
        drop(data);

        println!("Players: {}", tp);
        println!("Total Games: {}", tg);
        println!("Average Fairness: {}", fairness);
        thread::sleep(std::time::Duration::from_millis(1500));
    }
}
