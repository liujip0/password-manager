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
    let cli = CLI::parse();
    println!("Action: {:?}", cli.command);
}
