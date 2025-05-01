use hostname::get;
use minigrep::{Config, cmd_cat, cmd_cd, cmd_ls, cmd_mkdir, cmd_pwd, cmd_rm, cmd_touch};
use std::{
    env,
    io::{self, Write},
    process,
};
use users::get_current_username;

fn get_prompt() -> String {
    let username = get_current_username()
        .map(|u| u.to_string_lossy().into_owned())
        .unwrap_or_else(|| "user".into());

    let hostname = get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "host".into());

    let dir = std::env::current_dir()
        .ok()
        .and_then(|path| {
            let home = dirs::home_dir()?;
            Some(if path == home {
                "~".to_string()
            } else {
                path.file_name()?.to_string_lossy().to_string()
            })
        })
        .unwrap_or_else(|| "?".into());

    format!("\x1b[1;32m[{username}@{hostname} \x1b[1;34m{dir}]\x1b[0m$ ")
}

fn cli_loop() {
    loop {
        print!("{}", get_prompt());
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
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "pwd" => cmd_pwd(),
            "ls" => cmd_ls(),
            "touch" => {
                if parts.len() < 2 {
                    println!("Usage: touch <filename>");
                } else {
                    cmd_touch(parts[1]);
                }
            }
            "cat" => {
                if parts.len() < 2 {
                    println!("Usage: cat <filename>");
                } else {
                    cmd_cat(parts[1]);
                }
            }
            "mkdir" => {
                if parts.len() < 2 {
                    println!("Usage: mkdir <directory_name>");
                } else {
                    cmd_mkdir(parts[1]);
                }
            }
            "rm" => {
                if parts.len() < 2 {
                    println!("Usage: rm <file_or_dir>");
                } else {
                    cmd_rm(parts[1]);
                }
            }
            "cd" => {
                if parts.len() < 2 {
                    println!("Usage: cd <directory>");
                } else {
                    cmd_cd(parts[1]);
                }
            }
            //vim /nano
            "vim" | "nano" => {
                if parts.len() < 2 {
                    println!("Usage: {} <filename>", parts[0]);
                } else {
                    let editor = parts[0];
                    let filename = parts[1];
                    match std::process::Command::new(editor).arg(filename).status() {
                        Ok(status) if status.success() => {}
                        Ok(status) => println!("Editor exited with status: {}", status),
                        Err(e) => println!("Failed to launch editor: {}", e),
                    }
                }
            }
            _ => println!("Unknown command: {}", parts[0]),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
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
        println!("Welcome to minigrep CLI!");
        cli_loop();
    }
}
