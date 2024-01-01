use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    
    println!("Please input your guess");

    let mut guess = String::new();  

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //.read_line() returns a Result of type Enumeration or enum, enum == Err or Ok, pass enum into.expect() to handle error

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    print!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too small!"),
        Ordering::Equal => println!("You win!🥳🥳!"),
    }

    println!("the secret number is: {secret_number}");
}
