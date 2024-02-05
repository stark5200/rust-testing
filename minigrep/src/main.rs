use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
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
    fn build(args: &[String]) -> Result<Config, &'static str> {  
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        ok(Config { query, file_path })
    }
}