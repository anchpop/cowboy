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

    // Add counters to the global registry with different key types
    SHERIFF.register("counter", counter.clone());
    SHERIFF.register(42, counter.clone());

    // Access from anywhere using different key types
    let counter_1 = SHERIFF.get::<_, i32>("counter");
    let counter_2 = SHERIFF.get::<_, i32>(42); // Note: not &42

    *counter_1.w() += 1;
    *counter_2.w() += 2;

    // All counters should have the same value since they're all clones of the same original counter
    assert_eq!(counter_1, counter_2);
    println!("Counter: {counter}");
}
