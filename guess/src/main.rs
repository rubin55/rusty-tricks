use std::io;
use rand::Rng;

fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}. It was: {secret_number}")
}
