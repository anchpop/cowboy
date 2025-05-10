# Cowboy 🤠

A thread-safe wrapper for shared data with a focus on ergonomics.

Designed for quick prototyping, when you don't really want to think about lifetimes and just want to get something working as fast as possible.

## What is it?

`Cowboy<T>`  is just a wrapper for `Arc<RwLock<T>>` that's designed to be less ugly. It has more concise method names, and lots of trait implementations that make it work in more cases (you can use `Eq`, `Hash`, `Ord`, and so on as long as the underlying type supports it). 

## Features

- Thread-safe shared data with interior mutability.
- Simple API: just use `.r()` to read and `.w()` to write.
- Implements common traits like `Clone`, `PartialEq`, `Hash`, etc.
- Supports basic arithmetic operations.
- Unsafe methods for when you want to break the rules.
- `SHERIFF` for storing cowboys for later access. 

## Quick Start

```rust
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
```

I think we can all agree that you shouldn't use `Cowboy` or `SHERIFF` in production code, but I'm hopeful it can be useful for when you're prototyping and want the borrow checker to get out of your way.

## Examples

Check out the examples directory for more usage patterns:
- `basic.rs`: Simple usage
- `shared_data.rs`: Sharing data between threads
- `unsafe_methods.rs`: Demonstrating the unsafe methods

## License

MIT
