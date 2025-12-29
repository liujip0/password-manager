use std::path::PathBuf;

use rand::Rng;
use toml::Value;

use crate::storage::{self, write_to_file};

pub fn list(dir: &PathBuf) -> Result<(), String> {
    let mut passwords = storage::get_passwords_from_file(dir)?;

    passwords.remove(storage::VERSION_KEY);

    if passwords.is_empty() {
        println!("No passwords stored.");
        return Ok(());
    }

    println!("Stored passwords:");
    for key in passwords.keys() {
        println!("- {}", key);
    }
    Ok(())
}

pub fn get(dir: &PathBuf, key: &str) -> Result<(), String> {
    let passwords = storage::get_passwords_from_file(dir)?;

    match passwords.get(key) {
        Some(value) => {
            let value = match value {
                Value::String(s) => s,
                _ => {
                    return Err(format!("Password for key '{}' is not a valid string.", key));
                }
            };

            let value = match value.strip_prefix('"') {
                Some(v) => &v.to_string(),
                None => value,
            };
            let value = match value.strip_suffix('"') {
                Some(v) => &v.to_string(),
                None => value,
            };

            println!("Password for '{}':\n{}", key, value);
            Ok(())
        }
        None => Err(format!("No password found for key '{}'.", key)),
    }
}

pub fn set(dir: &PathBuf, key: &str, value: &str) -> Result<(), String> {
    let mut passwords = storage::get_passwords_from_file(dir)?;

    passwords.insert(key.to_string(), Value::String(value.to_string()));

    let write = storage::write_to_file(&dir, &passwords);
    match write {
        Err(e) => Err(format!(
            "Could not write updated passwords to file.\n\n{}",
            e
        )),
        Ok(_) => {
            println!("Password for '{}' set successfully.", key);
            Ok(())
        }
    }
}

pub fn generate(
    dir: &PathBuf,
    key: &str,
    special_chars: bool,
    length: usize,
) -> Result<(), String> {
    let alphanumeric_chars: Vec<char> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
            .to_string()
            .chars()
            .collect();
    let all_chars: Vec<char> = format!(
        "{}{}",
        alphanumeric_chars.iter().collect::<String>(),
        "!@#$%^&*()-_=+[]{};:,.?/ "
    )
    .chars()
    .collect();

    let chars_length = if special_chars {
        all_chars.len()
    } else {
        alphanumeric_chars.len()
    };
    let range = 0..chars_length;

    let mut rng = rand::rng();
    let password: String = (0..length)
        .map(|_| {
            if special_chars {
                all_chars[rng.random_range(range.clone())]
            } else {
                alphanumeric_chars[rng.random_range(range.clone())]
            }
        })
        .collect();

    let mut passwords = storage::get_passwords_from_file(dir)?;
    passwords.insert(key.to_string(), Value::String(password.clone()));
    write_to_file(dir, &passwords)?;

    println!("Generated and saved password for '{}':\n{}", key, password);
    Ok(())
}
