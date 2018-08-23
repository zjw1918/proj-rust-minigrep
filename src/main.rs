extern crate minigrep;

mod zjw_learn;

use std::env;
use minigrep::Config;
use std::process;

fn main() {
    println!("Hello, world >>>");
    // learn_minigrep();
    zjw_learn::run_learn_closure();
}

fn learn_minigrep() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("parse args error: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}\n", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("run error: {}", e);
        process::exit(1);
    }
}