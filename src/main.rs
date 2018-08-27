extern crate minigrep;

mod zjw_learn;

use std::env;
use minigrep::Config;
use std::process;
mod http_test;

#[macro_use]
extern crate serde_derive;

fn main() {
    println!("Hello, world >>>");
    // learn_minigrep();
    // zjw_learn::run_learn_closure(); // 2018-08-24 10:02:59 13 done
    // zjw_learn::run_learn_smart_pointer();
    // zjw_learn::run_learn_concurrent();
    // zjw_learn::run_learn_unsafe();
    // zjw_learn::run_learn_server();
    // zjw_learn::run_learn_http_request_json();
    http_test::run();
}

fn learn_minigrep() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("parse args error: {}", err);
        process::exit(1);
    });

    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("parse args error: {}", err);
    //     process::exit(1);
    // });

    println!("Searching for: {}", config.query);
    println!("In file: {}\n", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("run error: {}", e);
        process::exit(1);
    }
}