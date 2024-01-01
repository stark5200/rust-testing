use std::io;

fn main() {
    println!("Guess the number!");
    
    println!("Please input your guess");

    let mut guess = String::new();  

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //.read_line() returns a Result of type Enumeration or enum, enum == Err or Ok, pass enum into.expect() to handle error

    println!("You guessed: {guess}");
}
