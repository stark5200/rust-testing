fn main() {
    struct Car {
        brand: String, 
        miles: Number,
        price: Number,
    }
    enum IpAddrKind {
        V4,
        V6,
    }

    // match construct

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    // package containing multiple crates, library or binary crate, crate containing multiple modules, refer using path, use keyword, modules can be made public using pub.
    // new vector              let v: Vec<i32> = Vec::new();
    // macro               let v = vec![1, 2, 3];

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // either the String or the string slice &str types

    // hashmap

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);


    
    
}
