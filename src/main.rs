use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).expect("Invalid arguments");

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searhing for Q: {query}");
    println!("searhing for F: {file_path}");
    // dbg!(args);
}

