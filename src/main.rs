use minigrep::{cmd_cat, cmd_ls, cmd_pwd, cmd_touch, Config};
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
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Failed to read input.");
            continue;
        }

        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            //exit cmd
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            //pwd cmd
            "pwd" => cmd_pwd(),

            //ls cmd
            "ls" => cmd_ls(),

            //touch cmd
            "touch" => {
                if parts.len() < 2 {
                    println!("Usage: touch <filename>");
                } else {
                    cmd_touch(parts[1]);
                }
            }
            //cat cmd
            "cat" => {
                if parts.len() < 2 {
                    println!("Usage: cat <filename>");
                } else {
                    cmd_cat(parts[1]);
                }
            }

            //mkdir cmd 
            cmd if cmd.starts_with("mkdir ") => {
                let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    minigrep::cmd_mkdir(parts[1]);
                } else {
                    println!("Usage: mkdir <directory_name>");
                }
            },
            //rm cmd
            cmd if cmd.starts_with("rm ") => {
                let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    minigrep::cmd_rm(parts[1]);
                } else {
                    println!("Usage: rm <file_or_dir>");
                }
            },
            //cd cmd
            cmd if cmd.starts_with("cd ") => {
                let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    minigrep::cmd_cd(parts[1]);
                } else {
                    println!("Usage: cd <directory>");
                }
            },
            //error handling
            _ => println!("Unknown command: {}", input),
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
        cli_loop();
    }
}
