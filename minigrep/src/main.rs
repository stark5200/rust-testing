use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        //remove     println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    //let (query, file_path) = parse_config(&args);

    //remove     println!("searching for {}", config.query);
    //remove     println!("in file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        //remove     println!("Application error: {e}");
        process::exit(1);
    };
    //dbg!(args);

    /*
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    */
}

