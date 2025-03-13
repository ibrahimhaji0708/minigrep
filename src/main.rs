use std::{env, fs, process,};
use std::error::Error;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("with text:\n{}", contents);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arg: {}", err);
        process::exit(1);
});

    println!("searching for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = run(config) {
        println!("application error: {}", e);
        process::exit(1);
    }
}
