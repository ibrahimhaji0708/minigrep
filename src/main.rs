use minigrep::{cmd_cat, cmd_cd, cmd_ls, cmd_mkdir, cmd_pwd, cmd_rm, cmd_touch, Config};
use std::{
    env,
    io::{self, Write},
    process,
};

fn cli_loop() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }
        
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            // Exit commands
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            // PWD command
            "pwd" => cmd_pwd(),
            // LS command
            "ls" => cmd_ls(),
            // Touch command
            "touch" => {
                if parts.len() < 2 {
                    println!("Usage: touch <filename>");
                } else {
                    cmd_touch(parts[1]);
                }
            }
            // Cat command
            "cat" => {
                if parts.len() < 2 {
                    println!("Usage: cat <filename>");
                } else {
                    cmd_cat(parts[1]);
                }
            }
            // Mkdir command
            "mkdir" => {
                if parts.len() < 2 {
                    println!("Usage: mkdir <directory_name>");
                } else {
                    cmd_mkdir(parts[1]);
                }
            }
            // Rm command
            "rm" => {
                if parts.len() < 2 {
                    println!("Usage: rm <file_or_dir>");
                } else {
                    cmd_rm(parts[1]);
                }
            }
            // CD command
            "cd" => {
                if parts.len() < 2 {
                    println!("Usage: cd <directory>");
                } else {
                    cmd_cd(parts[1]);
                }
            }
            // Unknown command
            _ => println!("Unknown command: {}", parts[0]),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // File search mode
        let config = Config::new(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        
        println!("Searching for: {}", config.query);
        println!("In file: {}", config.filename);
        
        if let Err(e) = minigrep::run(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    } else {
        // Interactive CLI mode
        println!("Welcome to minigrep CLI!");
        cli_loop();
    }
}