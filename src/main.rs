use std::io;
use std::cmp::Ordering;
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

    let guess: u32 = guess.trim().parse().expect("Failed to parse, Please enter a Number");

    println!("Your guess is: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("This guess is less than the secret number"),
        Ordering::Equal => println!("This guess is equal to the secret number"),
        Ordering::Greater => println!("This guess is greater than the secret number"),
    }
}
