use std::ops::Add;

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("Yes: {}", add(52, 71));
}
