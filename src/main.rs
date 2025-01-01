mod db;
mod in_memory_db;

use std::io::Write;

use clap::{Parser, Subcommand};
use db::{Database, DbRecord};
use in_memory_db::InMemoryDb;

fn main() -> Result<(), String> {
    let mut db = InMemoryDb::new().with_engine("hashmap");
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(&mut db, line) {
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

fn respond(db: &mut InMemoryDb, line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
    match cli.command {
        Commands::Version => {
            db.version();
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Commands::Exit => {
            write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        }
        Commands::Get { key } => {            
            show_record(db.get(key));
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Commands::Del { key } => {           
            show_record(db.del(key));
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Commands::Set { key, value } => {
            show_record(db.put(key, value));
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
    Del { key: String },
    Set { key: String, value: String },
    Version,
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

fn show_record(result: Option<DbRecord>) {
    match result {
        Some(v) => {
            println!("Key: {} and Value : {}", v.key, v.value);
        }
        None => {
            println!("Operation failed. Please try again.");
        }
    }
}
