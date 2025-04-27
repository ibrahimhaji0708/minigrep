use std::{env, process, io::{self, Write}};
use minigrep::{Config, cmd_pwd, cmd_ls};

fn cli_loop() {  // <-- move cli_loop above
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Failed to read input.");
            continue;
        }

        let command = input.trim();
        match command {
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            },
            "pwd" => {
                cmd_pwd(); // <-- no need minigrep::cmd_pwd()
            },
            "ls" => {
                cmd_ls(); // <-- same here
            },
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let config = Config::new(&args).unwrap_or_else(|err| {
            eprintln!("problem parsing args: {}", err);
            process::exit(1);
        });

        println!("Searching for: {}", config.query);
        println!("In file: {}", config.filename);

        if let Err(e) = minigrep::run(config) {
            eprintln!("app error: {}", e);
            process::exit(1);
        }
    } else {
        println!("Welcome to minigrep CLI!..");
        cli_loop();  // now no error
    }
}
