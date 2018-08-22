extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Parsing args error: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("App error: {}", e);
        process::exit(1);
    }
}