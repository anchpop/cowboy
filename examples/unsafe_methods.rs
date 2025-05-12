#![allow(deprecated)]

fn main() {
    println!("🚨 DANGER ZONE 🚨");
    println!("This example demonstrates the unsafe methods howdy() and yeehaw()");
    println!("These methods are for prototyping and will probably lead to undefined behavior!\n");

    #[cfg(not(feature = "evil"))]
    {
        println!("🚨 This example requires the `evil` feature to be enabled. 🚨");
    }

    #[cfg(feature = "evil")]
    {
        use cowboy::*;
        // Create a shared counter
        let counter = 42.cowboy();

        // Using howdy() to get a reference without locking
        let value_ref: &i32 = unsafe { counter.howdy() };
        println!("Value via howdy(): {}", value_ref);

        // Using yeehaw() to get a mutable reference without proper locking
        let value_mut: &mut i32 = unsafe { counter.yeehaw() };
        *value_mut = 100;
        println!("Modified value via yeehaw() to: {}", value_mut);

        // Verify the change using the safe API
        println!("Value via safe r(): {}", *counter.r());

        // This is undefined behavior!
        println!("\n🚨 UNDEFINED BEHAVIOR DEMONSTRATION 🚨");
        let ref1: &mut i32 = unsafe { counter.yeehaw() };
        let ref2: &mut i32 = unsafe { counter.yeehaw() }; // This creates a second mutable reference

        // Modifying through both references
        *ref1 = 200;
        *ref2 = 300; // This is undefined behavior!

        println!("After undefined behavior, value is: {}", *counter.r());
        println!("\nNote: The actual behavior you observe may vary and is not guaranteed.");
        println!("In a real application, this could lead to memory corruption or crashes.");
    }
}
