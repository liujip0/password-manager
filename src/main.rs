use clap::{ArgAction, Parser, Subcommand};

mod commands;
mod storage;

#[derive(Parser)]
#[command(version, about, author)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    List,
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Generate {
        key: String,
        #[arg(short, long = "special", default_value_t = true, action = ArgAction::SetFalse)]
        special_chars: bool,
        #[arg(short, long, default_value_t = 32)]
        length: usize,
    },
}

fn main() {
    let cli = CLI::try_parse();
    let cli = match cli {
        Err(e) => {
            eprintln!("Error parsing command line arguments\n\n{}", e);
            std::process::exit(1);
        }
        Ok(cli) => cli,
    };

    let home_dir = std::env::home_dir();
    let home_dir = match home_dir {
        Some(dir) => dir,
        None => {
            eprintln!("Could not determine home directory");
            std::process::exit(1);
        }
    };

    println!("Action: {:?}", cli.command);

    match cli.command {
        Commands::List => {
            let list = commands::list(&home_dir);
            match list {
                Err(e) => {
                    eprintln!("Error listing passwords\n\n{}", e);
                    std::process::exit(1);
                }
                Ok(_) => {}
            }
        }
        _ => {
            println!("Command not implemented yet.");
        }
    }
}
