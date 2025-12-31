use std::fmt::{self};

use clap::{ArgAction, Parser, Subcommand, ValueEnum};

use crate::logs::write_to_log;

mod autocomplete;
mod backup;
mod commands;
mod logs;
mod password;
mod storage;

#[derive(Parser)]
#[command(version, about, author)]
#[command(help_template = "\
    {name} v{version}    {author}\n\n\
    {usage-heading}\n    {usage}\n\n\
    {about-with-newline}\n\
    {all-args}{after-help}")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

const SUBCOMMAND_HELP_TEMPLATE: &str = "\
    {name}\n\n\
    {about}\n\n\
    {usage-heading}\n    {usage}\n\n\
    {all-args}{after-help}";
#[derive(Debug, Subcommand)]
enum Commands {
    /// List the keys of all stored passwords
    #[clap(aliases = ["ls", "l"], help_template = SUBCOMMAND_HELP_TEMPLATE)]
    List,
    /// Get a stored password by its key
    #[clap(aliases = ["g"], help_template = SUBCOMMAND_HELP_TEMPLATE)]
    Get {
        key: Option<String>,
        #[arg(short, long = "master")]
        master_password: Option<String>,
    },
    /// Set a password for a given key
    #[clap(aliases = ["s", "add"], help_template = SUBCOMMAND_HELP_TEMPLATE)]
    Set {
        key: Option<String>,
        value: Option<String>,
        #[arg(short, long = "master")]
        master_password: Option<String>,
    },
    /// Generate a random password and store it with the given key
    #[clap(aliases = ["gen"], help_template = SUBCOMMAND_HELP_TEMPLATE)]
    Generate {
        key: Option<String>,
        /// Whether to include !@#$%^& *()-_=+[]{};:,.?/ in the generated password
        #[arg(short, long = "special", action=ArgAction::Set)]
        special_chars: Option<bool>,
        /// Length in characters of the generated password
        #[arg(short, long)]
        length: Option<usize>,
        #[arg(short, long = "master")]
        master_password: Option<String>,
    },
    /// Export all stored passwords to a plaintext file
    #[clap(aliases = ["exp", "ex", "backup", "out"], help_template = SUBCOMMAND_HELP_TEMPLATE)]
    Export {
        #[arg(short, long = "file")]
        file_path: Option<String>,
        #[arg(short = 't', long = "type")]
        file_type: Option<ExportType>,
        #[arg(short, long = "master")]
        master_password: Option<String>,
    },
    /// Import passwords from a file
    #[clap(aliases = ["in", "restore", "load"], help_template = SUBCOMMAND_HELP_TEMPLATE)]
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

fn main() {
    let home_dir = std::env::home_dir();
    let Some(home_dir) = home_dir else {
        println!("Could not determine home directory.");
        return;
    };

    write_to_log(
        &home_dir,
        format!("Application started. Version {}", env!("CARGO_PKG_VERSION")).as_str(),
    )
    .unwrap_or(());

    let cli = CLI::try_parse();
    let cli = match cli {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(cli) => cli,
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
    };
    if let Err(e) = result {
        write_to_log(&home_dir, format!("Error occurred: {}", &e).as_str()).unwrap_or(());
        println!("{}", e);
    };
}
