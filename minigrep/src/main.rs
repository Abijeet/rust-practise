extern crate minigrep;

use std::env;
use std::process;

fn main() {
    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem while parsing the arguments: {}", err);
        process::exit(1);
    });

    println!("Search query: {}", config.query);
    println!("In file: {}", config.filename);
    println!("------------------------");
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
