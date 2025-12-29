use std::{
    fs::{self, File},
    path::PathBuf,
};

use inquire::Confirm;
use toml::{Table, Value};

const PASSWORDS_FILE: &str = "liujip0-password-manager.toml";
pub(crate) const VERSION_KEY: &str = "__PASSWORD_MANAGER_VERSION__";

pub fn get_passwords_from_file(dir: &PathBuf) -> Result<Table, String> {
    let file_path = dir.join(PASSWORDS_FILE);
    create_file_if_not_exists(&dir)?;

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
    let passwords = match passwords {
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
        return Ok(passwords);
    } else {
        let file_version = passwords.get(VERSION_KEY);
        let file_version = match file_version {
            Some(Value::String(v)) => v,
            _ => "[unknown version]",
        };

        println!(
            "Warning: Passwords file version ({}) does not match application version ({}).",
            file_version,
            env!("CARGO_PKG_VERSION")
        );

        let remake_file = remake_bad_file(&dir);
        match remake_file {
            Err(e) => return Err(e),
            Ok(_) => {
                print!("Retrying...");
                return get_passwords_from_file(dir);
            }
        }
    }
}

fn remake_bad_file(dir: &PathBuf) -> Result<(), String> {
    let file_path = dir.join(PASSWORDS_FILE);

    println!(
        "The passwords file at {} is corrupted.",
        file_path.display()
    );

    let recreate_file = Confirm::new("Recreate the passwords file?")
        .with_default(false)
        .with_help_message("All stored passwords will be lost.")
        .prompt();
    let recreate_file = match recreate_file {
        Err(e) => {
            return Err(format!(
                "Could not get user confirmation to recreate passwords file\n\n{}",
                e
            ));
        }
        Ok(choice) => choice,
    };

    if !recreate_file {
        println!("Not recreating passwords file. Quitting...");
        return Err("Corrupted passwords file.".to_string());
    }

    println!("Recreating passwords file at: {}", &file_path.display());
    let remove_file = fs::remove_file(&file_path);
    match remove_file {
        Err(e) => {
            return Err(format!(
                "Could not remove corrupted passwords file at {}\n\n{}",
                &file_path.display(),
                e
            ));
        }
        Ok(_) => {
            println!("Corrupted passwords file removed.");
        }
    };

    create_file_if_not_exists(dir)
}

fn create_file_if_not_exists(dir: &PathBuf) -> Result<(), String> {
    let file_path = dir.join(PASSWORDS_FILE);

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
    let file = File::create_new(&file_path);
    match file {
        Err(e) => {
            return Err(format!(
                "Could not create passwords file at {}\n\n{}",
                &file_path.display(),
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

    write_to_file(&dir, &init_config)
}

pub fn write_to_file(dir: &PathBuf, contents: &Table) -> Result<(), String> {
    let file_path = dir.join(PASSWORDS_FILE);

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

    let write = fs::write(&file_path, contents);
    match write {
        Err(e) => {
            return Err(format!(
                "Could not write contents to file at {}\n\n{}",
                &file_path.display(),
                e
            ));
        }
        Ok(_) => return Ok(()),
    };
}
