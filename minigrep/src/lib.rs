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
    let mut f = File::open(&config.filename)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;
    let results = search(&content, &config.query);

    if results.len() == 0 {
        println!("Search string {} not found in file - {}.", config.query, config.filename);
        return Ok(());
    }

    for result in results {
        println!("#{}: {}", result.line_num, result.line);
    }
    Ok(())
}

pub struct SearchResult <'a> {
    pub line_num: i32,
    pub line: &'a str
}

pub fn search<'a>(content: &'a str, query: &str) -> Vec<SearchResult <'a>> {
    let mut line_num = 1;
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(SearchResult {
                line_num: line_num,
                line: line
            })
        }
        line_num += 1;
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn does_not_match() {
        let content = "\
            Rust is a 
            static, concurrent, parallel,
            programming language.
            That kicks ass!!
        ";
        let query = "Rust";
        let search_results: Vec<SearchResult> = search(content, query);
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].line_num, 1);
    }
}