#![allow(deprecated)]

fn main() {
    println!("🚨 DANGER ZONE 🚨");
    println!("This example demonstrates the unsafe methods howdy() and yehaw()");
    println!("These methods are for prototyping and will probably lead to undefined behavior!\n");

    #[cfg(not(feature = "evil"))]
    {
        println!("🚨 This example requires the `evil` feature to be enabled. 🚨");
    }

    #[cfg(feature = "evil")]
    {
        use cowboy::cowboy;
        // Create a shared counter
        let counter = cowboy(42);

        // Using howdy() to get a reference without locking
        let value_ref: &i32 = counter.howdy();
        println!("Value via howdy(): {}", value_ref);

        // Using yehaw() to get a mutable reference without proper locking
        let value_mut: &mut i32 = counter.yehaw();
        *value_mut = 100;
        println!("Modified value via yehaw() to: {}", value_mut);

        // Verify the change using the safe API
        println!("Value via safe r(): {}", *counter.r());

        // This is undefined behavior!
        println!("\n🚨 UNDEFINED BEHAVIOR DEMONSTRATION 🚨");
        let ref1: &mut i32 = counter.yehaw();
        let ref2: &mut i32 = counter.yehaw(); // This creates a second mutable reference

        // Modifying through both references
        *ref1 = 200;
        *ref2 = 300; // This is undefined behavior!

        println!("After undefined behavior, value is: {}", *counter.r());
        println!("\nNote: The actual behavior you observe may vary and is not guaranteed.");
        println!("In a real application, this could lead to memory corruption or crashes.");
    }
}
