use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).expect("Invalid arguments");

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Should have been able to read the file");

    println!("File contents:\n{}", contents);
}
