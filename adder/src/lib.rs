pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")//format!("Hello {}!", name)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        } 
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8, 
            height: 7, 
        };
        let smaller = Rectangle {
            width: 6, 
            height: 2, 
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 10, 
            height: 7, 
        };
        let smaller = Rectangle {
            width: 3, 
            height: 5, 
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn failed() {
        panic!("make this test fail");
    }

    #[test] 
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test] 
    fn greeting_contains_name() {
        let result = greeting("Stark");
        println!("Passin test prints line");
        assert!(
            result.contains("Stark"), 
            "Greeting did not contain name, value was `{}`", 
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")] 
    fn greater_than_100() {
        Guess::new(-200);
    }
}
