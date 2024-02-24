use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //let (query, file_path) = parse_config(&args);
    //remove     println!("searching for {}", config.query);
    //remove     println!("in file {}", config.file_path);
    //dbg!(args);
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}

