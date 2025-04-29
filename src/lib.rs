use std::error::Error;
use std::fs::File;
use std::{fs, env};
// pub use self::*;
// use std::path::Path;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool, 
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

pub fn cmd_pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn cmd_ls() {
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        println!("{}/", path.file_name().unwrap().to_string_lossy());
                    } else {
                        println!("{}", path.file_name().unwrap().to_string_lossy());
                    }
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

//touch cmd
pub fn cmd_touch(filename: &str) {
    match File::create(filename) {
        Ok(_) => println!("File '{}' created.", filename),
        Err(e) => println!("Failed to create file '{}': {}", filename, e),
    }
}

//cat cmd
pub fn cmd_cat(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Error reading file '{}': {}", filename, e),
    }
}
