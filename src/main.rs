use clap::{ArgAction, Parser, Subcommand};

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
    println!("Action: {:?}", cli.command);
}
