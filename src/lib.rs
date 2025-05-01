use std::error::Error;
use std::fs::{read_to_string, File};
use std::path::Path;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
    pub recursive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let recursive = args.get(1).map(|s| s == "-r").unwrap_or(false);
        let (query, filename) = if recursive {
            (args[2].clone(), args[3].clone())
        } else {
            (args[1].clone(), args[2].clone())
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive, recursive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.recursive {
        recursive_search(&config.query, &config.filename, config.case_sensitive)?;
    } else {
        let contents = read_to_string(&config.filename)?;
        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in results {
            println!("{}", line);
        }
    }
    Ok(())
}
fn recursive_search(query: &str, dir: &str, case_sensitive: bool) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            recursive_search(query, &path.to_string_lossy(), case_sensitive)?;
        } else if path.is_file() {
            let content = fs::read_to_string(&path);
            if let Ok(content) = content {
                let results = if case_sensitive {
                    search(query, &content)
                } else {
                    search_case_insensitive(query, &content)
                };
                if !results.is_empty() {
                    println!("{}:", path.display());
                    for line in results {
                        println!("{}", line);
                    }
                    println!();
                }
            }
        }
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
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    
    results
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

pub fn cmd_touch(filename: &str) {
    match File::create(filename) {
        Ok(_) => println!("File '{}' created.", filename),
        Err(e) => println!("Failed to create file '{}': {}", filename, e),
    }
}

pub fn cmd_cat(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Error reading file '{}': {}", filename, e),
    }
}

pub fn cmd_mkdir(name: &str) {
    match fs::create_dir(name) {
        Ok(_) => println!("Directory '{}' created.", name),
        Err(e) => println!("Failed to create directory '{}': {}", name, e),
    }
}

pub fn cmd_rm(target: &str) {
    let path = Path::new(target);
    
    if path.is_dir() {
        match fs::remove_dir_all(path) {
            Ok(_) => println!("Directory '{}' removed.", target),
            Err(e) => println!("Error removing directory '{}': {}", target, e),
        }
    } else if path.is_file() {
        match fs::remove_file(path) {
            Ok(_) => println!("File '{}' removed.", target),
            Err(e) => println!("Error removing file '{}': {}", target, e),
        }
    } else {
        println!("No such file or directory: '{}'", target);
    }
}

pub fn cmd_cd(path: &str) {
    match env::set_current_dir(path) {
        Ok(_) => (),
        Err(e) => println!("Failed to change directory to '{}': {}", path, e),
    }
}