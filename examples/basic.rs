use cowboy::cowboy;

fn main() {
    // Create a new Cowboy with a simple value
    let data = cowboy(42);

    // Read the value
    {
        let value = data.r();
        println!("The value is: {}", *value);
    }

    // Modify the value
    {
        let mut value = data.w();
        *value = 100;
        println!("Updated value to: {}", *value);
    }

    // Read again to confirm the change
    {
        let value = data.read();
        println!("The value is now: {}", *value);
    }
}
