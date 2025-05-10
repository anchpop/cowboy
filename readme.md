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
use cowboy::{cowboy, SHERIFF};

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
```

I think we can all agree that you shouldn't use `Cowboy` or `SHERIFF` in production code, but I'm hopeful it can be useful for when you're prototyping and want the borrow checker to get out of your way. 

## Examples

Check out the examples directory for more usage patterns:
- `basic.rs`: Simple usage
- `shared_data.rs`: Sharing data between threads
- `unsafe_methods.rs`: Demonstrating the unsafe methods

## License

MIT
