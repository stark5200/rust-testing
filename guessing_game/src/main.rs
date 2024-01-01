use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    
    println!("Please input your guess");

    let mut guess = String::new();  

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //.read_line() returns a Result of type Enumeration or enum, enum == Err or Ok, pass enum into.expect() to handle error

    print!("You guessed: {guess}");
    println!("the secret number is: {secret_number}");
}
