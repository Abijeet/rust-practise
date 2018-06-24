use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Missing arguments.");
        }
        let filename = args[1].clone();
        let query = args[2].clone();
        Ok(Config { 
            filename,
            query
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;

    println!("File contents: {}", content);
    Ok(())
}