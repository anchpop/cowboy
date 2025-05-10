use cowboy::{SHERIFF, cowboy};
use std::thread;
use std::time::Duration;

// A simple player struct for demonstration
#[derive(Debug, Clone)]
struct Player {
    name: String,
    score: i32,
}

impl Player {
    fn new() -> Self {
        Self {
            name: "Unnamed".to_string(),
            score: 0,
        }
    }

    fn shoot(&mut self) -> i32 {
        self.score += 10;
        println!("{} shot! Score: {}", self.name, self.score);
        self.score
    }
}

fn main() {
    println!("Demonstrating global access to Cowboy instances\n");

    // Register a player globally
    SHERIFF.register("player1", cowboy(Player::new()));

    // Modify the player through the global registry
    {
        let player = SHERIFF.get::<Player>("player1");
        player.w().name = "Gunslinger".to_string();
        player.w().shoot();
    }

    // Create a thread that accesses the player
    let handle = thread::spawn(move || {
        // Access the player from anywhere
        let player = SHERIFF.get::<Player>("player1");
        for _ in 1..=3 {
            let mut player_write = player.w();
            player_write.shoot();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Wait for the thread to complete
    handle.join().unwrap();

    // Access the player again from the main thread
    let player = SHERIFF.get::<Player>("player1");
    let player_read = player.r();
    println!("\nFinal player state:");
    println!("Name: {}", player_read.name);
    println!("Score: {}", player_read.score);
}
