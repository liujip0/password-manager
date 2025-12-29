use clap::{ArgAction, Parser, Subcommand};

mod commands;
mod password;
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
        key: Option<String>,
        #[arg(short, long = "master")]
        master_password: Option<String>,
    },
    Set {
        key: Option<String>,
        value: Option<String>,
        #[arg(short, long = "master")]
        master_password: Option<String>,
    },
    Generate {
        key: Option<String>,
        #[arg(short, long = "special", action=ArgAction::Set)]
        special_chars: Option<bool>,
        #[arg(short, long)]
        length: Option<usize>,
        #[arg(short, long = "master")]
        master_password: Option<String>,
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

    let result = match cli.command {
        Commands::List => commands::list(&home_dir),
        Commands::Get {
            key,
            master_password,
        } => commands::get(&home_dir, &key, &master_password),
        Commands::Set {
            key,
            value,
            master_password,
        } => commands::set(&home_dir, &key, &value, &master_password),
        Commands::Generate {
            key,
            special_chars,
            length,
            master_password,
        } => commands::generate(&home_dir, &key, special_chars, length, &master_password),
    };
    match result {
        Err(e) => {
            eprintln!("Error executing command\n\n{}", e);
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}
