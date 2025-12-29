use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use inquire::Confirm;
use toml::{Table, Value};

const PASSWORDS_FILE: &str = "liujip0-password-manager.toml";
const VERSION_KEY: &str = "__PASSWORD_MANAGER_VERSION__";

pub fn get_passwords_from_file(dir: &PathBuf) -> Result<Table, String> {
    let file_path = dir.join(PASSWORDS_FILE);
    create_file_if_not_exists(&file_path)?;

    let file = fs::read_to_string(&file_path);
    let file = match file {
        Err(e) => {
            return Err(format!(
                "Could not read passwords file at {}\n\n{}",
                file_path.display(),
                e
            ));
        }
        Ok(content) => content,
    };

    let passwords = toml::from_str::<Table>(&file);
    let mut passwords = match passwords {
        Err(e) => {
            return Err(format!(
                "Could not parse passwords file at {} as TOML format\n\n{}",
                file_path.display(),
                e
            ));
        }
        Ok(table) => table,
    };

    if passwords.get(VERSION_KEY) == Some(&Value::String(env!("CARGO_PKG_VERSION").to_string())) {
        passwords.remove(VERSION_KEY);
        return Ok(passwords);
    } else {
        println!(
            "Warning: Passwords file version ({}) does not match application version ({}).",
            passwords[VERSION_KEY],
            env!("CARGO_PKG_VERSION")
        );
        return Err("Passwords file version mismatch.".to_string())?;
    }
}

fn create_file_if_not_exists(file_path: &PathBuf) -> Result<(), String> {
    if file_path.exists() {
        println!("Passwords file exists at: {}", file_path.display());
        return Ok(());
    }

    println!("Passwords file does not exist.");

    let create_new_file = Confirm::new("Create a new passwords file?")
        .with_default(true)
        .with_help_message(format!("The file will be created at {}", file_path.display()).as_str())
        .prompt();
    let create_new_file = match create_new_file {
        Err(e) => {
            return Err(format!(
                "Could not get user confirmation to create passwords file\n\n{}",
                e
            ));
        }
        Ok(choice) => choice,
    };

    if !create_new_file {
        println!("Passwords file does not exist. Quitting...");
        return Err("Passwords file does not exist.".to_string());
    }

    println!("Creating passwords file at: {}", file_path.display());
    let file = File::create_new(file_path);
    let mut file = match file {
        Err(e) => {
            return Err(format!(
                "Could not create passwords file at {}\n\n{}",
                file_path.display(),
                e
            ));
        }
        Ok(file) => {
            println!("Passwords file created at: {}", file_path.display());
            file
        }
    };

    let mut init_config = Table::new();
    init_config.insert(
        VERSION_KEY.to_string(),
        Value::String(env!("CARGO_PKG_VERSION").to_string()),
    );

    write_to_file(file_path, &init_config)
}

fn write_to_file(file_path: &PathBuf, contents: &Table) -> Result<(), String> {
    let contents = toml::to_string_pretty(contents);
    let contents = match contents {
        Err(e) => {
            return Err(format!(
                "Could not serialize contents to TOML format\n\n{}",
                e
            ));
        }
        Ok(contents) => contents,
    };

    let write = fs::write(file_path, contents);
    match write {
        Err(e) => {
            return Err(format!(
                "Could not write contents to file at {}\n\n{}",
                file_path.display(),
                e
            ));
        }
        Ok(_) => return Ok(()),
    };
}
