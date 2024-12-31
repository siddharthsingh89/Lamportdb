use std::io::Write;

use clap::{Parser, Subcommand};

fn main() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
    match cli.command {
        Commands::Version => {
            show_version();
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        },
        Commands::Exit => {
            write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        }
        Commands::Get { key } => {
            get_record(key);
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Commands::Set { key, value } => {
            set_record(key, value);
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
    }
    Ok(false)
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Exit,
    Get { key: String },
    Set { key: String, value: String },
    Version
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

fn set_record(key: String, value: String) {
    println!(
        "Record with key {} and value{} is written to database",
        key, value
    );
}

fn get_record(key: String) {
    println!("Record with key {} is fetch from database", key);
}

fn show_version() {
    println!("Lamport DB version 0.1.0")
}
