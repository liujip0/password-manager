use std::fmt::{self};

use clap::{ArgAction, Parser, Subcommand, ValueEnum};

mod autocomplete;
mod backup;
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
    Export {
        #[arg(short, long = "file")]
        file_path: Option<String>,
        #[arg(short = 't', long = "type")]
        file_type: Option<ExportType>,
        #[arg(short, long = "master")]
        master_password: Option<String>,
    },
    Import {
        #[arg(short, long = "file")]
        file_path: Option<String>,
        #[arg(short, long = "master")]
        master_password: Option<String>,
        #[arg(short, long, action=ArgAction::Set)]
        overwrite: Option<bool>,
    },
}
#[derive(Debug, Clone, ValueEnum)]
enum ExportType {
    Json,
    Csv,
    Toml,
}
impl fmt::Display for ExportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExportType::Json => "json",
            ExportType::Csv => "csv",
            ExportType::Toml => "toml",
        };
        write!(f, "{}", s)
    }
}

fn main() -> Result<(), String> {
    let cli = CLI::try_parse();
    let cli = match cli {
        Err(e) => {
            return Err(format!("Could not parse command line arguments.\n\n{}", e));
        }
        Ok(cli) => cli,
    };

    let home_dir = std::env::home_dir();
    let Some(home_dir) = home_dir else {
        return Err("Could not determine home directory".to_string());
    };

    match cli.command {
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
        Commands::Export {
            file_path,
            file_type,
            master_password,
        } => backup::export_to_file(&home_dir, &file_path, &file_type, &master_password),
        Commands::Import {
            file_path,
            master_password,
            overwrite,
        } => backup::import_from_file(&home_dir, &file_path, &master_password, overwrite),
    }
}
