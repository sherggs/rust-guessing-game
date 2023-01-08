use std::io;
use rand::Rng;

fn main() {
    println!("Guess a random number! ");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Your secret number is: {secret_number}");


    println!("Please input your guess number ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    println!("Your guess is: {guess}");
}
