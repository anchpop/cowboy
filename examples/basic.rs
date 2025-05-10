fn main() {
    use cowboy::{SHERIFF, cowboy};

    // Create a shared counter
    let counter = cowboy(0);
    let counter_2 = counter.clone();

    println!("Counter: {counter}");

    // Modify the value
    {
        let mut value = counter.w();
        *value += 1;
    }

    assert_eq!(counter, counter_2);

    // Add the counter to the global registry
    SHERIFF.register("counter", counter);
    // Access from anywhere
    let counter = SHERIFF.get::<i32>("counter");
    *counter.w() += 1;
}
