use cowboy::*;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a shared counter
    let counter = 0.cowboy();

    // Clone the Cowboy for different threads
    let counter1 = counter.clone();
    let counter2 = counter.clone();

    // Thread 1: Increment the counter every 100ms
    let handle1 = thread::spawn(move || {
        for _ in 0..5 {
            {
                let mut value = counter1.w();
                *value += 1;
                println!("Thread 1: Incremented to {}", *value);
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Thread 2: Read the counter every 150ms
    let handle2 = thread::spawn(move || {
        for i in 0..5 {
            {
                let value = counter2.r();
                println!("Thread 2: Read value {} (iteration {})", *value, i + 1);
            }
            thread::sleep(Duration::from_millis(150));
        }
    });

    // Wait for both threads to complete
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Final value
    println!("Final counter value: {}", *counter.r());
}
