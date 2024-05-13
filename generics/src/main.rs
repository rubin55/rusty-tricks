use std::ops::Add;
use std::time::Duration;
fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a.add(b) // is the same as a + b
}

fn main() {
    println!("Durations: {:?}", add(Duration::new(5, 0), Duration::new(10, 0)));
    println!("Integers: {}", add(52, 71));
    println!("Floats: {}", add(3.3, 4.6));
}
