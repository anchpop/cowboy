fn main() {
    #[cfg(not(feature = "serde"))]
    {
        println!("This example requires the `serde` feature");
    }

    #[cfg(feature = "serde")]
    {
        use cowboy::*;

        let counter = 0.cowboy();

        counter.save("counter.json");

        let new_counter = Cowboy::<i64>::load("counter.json");

        println!("Counter: {new_counter}");
    }
}
