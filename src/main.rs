use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searhing for Q: {query}");
    println!("searhing for F: {file_path}");
    // dbg!(args);
}

