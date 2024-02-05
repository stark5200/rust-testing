use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    //let (query, file_path) = parse_config(&args);

    println!("searching for {}", config.query);
    println!("in file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should be able to read file");

    println!("With text:\n{contents}");
    //dbg!(args);
}

struct Config {
    query: String, 
    file_path: String, 
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}