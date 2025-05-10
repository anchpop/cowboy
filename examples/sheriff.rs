use cowboy::*;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PlayerId(u32);

fn main() {
    println!("Demonstrating global access to Cowboy instances\n");

    // Register players with different key types
    SHERIFF.register("player1", Player::new().cowboy()); // String key
    SHERIFF.register(PlayerId(2), Player::new().cowboy()); // Custom type key
    SHERIFF.register(42, Player::new().cowboy()); // Integer key

    // Modify players through the global registry
    {
        // String key
        let player = SHERIFF.get::<_, Player>("player1");
        player.w().name = "Gunslinger".to_string();
        player.w().shoot();

        // Custom type key
        let player2 = SHERIFF.get::<_, Player>(&PlayerId(2));
        player2.w().name = "Sharpshooter".to_string();
        player2.w().shoot();

        // Integer key
        let player3 = SHERIFF.get::<_, Player>(&42);
        player3.w().name = "Desperado".to_string();
        player3.w().shoot();
    }

    // Create a thread that accesses the player with custom key
    let handle = thread::spawn(move || {
        // Access the player from anywhere
        let player = SHERIFF.get::<_, Player>(&PlayerId(2));
        for _ in 1..=3 {
            let mut player_write = player.w();
            player_write.shoot();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Wait for the thread to complete
    handle.join().unwrap();

    // Access all players again from the main thread
    println!("\nFinal player states:");

    let player1 = SHERIFF.get::<_, Player>("player1");
    let player1_read = player1.r();
    println!("Player1 (String key):");
    println!("  Name: {}", player1_read.name);
    println!("  Score: {}", player1_read.score);

    let player2 = SHERIFF.get::<_, Player>(&PlayerId(2));
    let player2_read = player2.r();
    println!("Player2 (PlayerId key):");
    println!("  Name: {}", player2_read.name);
    println!("  Score: {}", player2_read.score);

    let player3 = SHERIFF.get::<_, Player>(&42);
    let player3_read = player3.r();
    println!("Player3 (Integer key):");
    println!("  Name: {}", player3_read.name);
    println!("  Score: {}", player3_read.score);
}
