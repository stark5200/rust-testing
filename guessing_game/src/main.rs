use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    
    loop {
        println!("Please input your guess");

        let mut guess = String::new();  

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //.read_line() returns a Result of type Enumeration or enum, enum == Err or Ok, pass enum into.expect() to handle error

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Great!"),
            Ordering::Equal => {
                println!("You win!🥳🥳!");
                break;
            }
        }
    }
    println!("the secret number is: {secret_number}");
}
