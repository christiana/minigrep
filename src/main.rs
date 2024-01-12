use std::env;
use std::process;
//use std::error::Error;
//use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    println!("======");

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
