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

/*
 * additional notes:
 * Rust is statically typed languange
 * const has global scobe unlike variables 
 * use unmutable var almost as if they are const inside sertain scope 
 * example : { let age = 25;} this lets variable act as const only inside this scope and can't be changed
 * const needs to be annotated example: {const THREE_SECONDS: u32 = 3;}
 * shadowing can be used as a step between mut and unmut var
 * example: {
 *     let x = 5; let x = x + 2; // this is called shadowing and is valid, x becomes 7
 *     x = x + 3; // this is not valid x is immutable
 *     { let x = x * 2; println!({x}) // value of x is now 14 }
 *     prinln!(x) // out of scope from previous shodowed version, x is still 7
 *     // shadowing also allows us to change types
 *     let spaces = "   "; let spaces = spaces.len(); // posssible
 *     let mut spaces = "   "; spaces = spaces.len(); // not posssible
 * }
 * tuples: // annotated types but can be different and fixed length
 * let tup: (i32, f64, u8, isize, char, string, bool) = (98_222, 3.5, b'A', 0xffffffff, '😎', "Hello darkness", false);
 * let (a, b, c, d, e, f, g) = tup;
 * prinln!((a, b, c, d, e, f, g) === (tup.0, tup.1, tup.2 ,tup.3 ,tup.4 ,tup.5 ,tup.6)) // prints true
 * tuples: // fixed type and length
 * let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
 * let a: [i32; 5] = [1, 2, 3, 4, 5];
 * let a = [3; 5]; //same as let a = [3, 3, 3, 3, 3];
 * access elements: let first = a[0];


use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
} 

// Chapter 4: Ownership

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
    // doesn't work, both strings are stored in the heap unlike other variables, when s1 and s2 go out of scope memory manager will try to clear the same heap twice when in fact s1 and s2 point to the same place in the heap causing double free error, to avoid this problem Rust considers s1 no longer valid, in other words s1 is moved into s2.

    let s1 = 5;
    let s2 = s1;
    println!("{}, world!", s1);
    // This works

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // deep copying or cloning

    fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}


*/
