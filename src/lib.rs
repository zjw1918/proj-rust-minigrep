use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

// 老版 单独的普通函数
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config { query, filename }
// }

// 新版，与结构体关联的new函数
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

// old run
// fn run(config: Config) {
//     let mut f = File::open(config.filename).expect("file not found");
//     let mut contents = String::new();
//     f.read_to_string(&mut contents)
//         .expect("reading file error");
//     println!("With text: \n{}", contents);
// }

// new run
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("With text: \n{}", contents);
    Ok(())
}