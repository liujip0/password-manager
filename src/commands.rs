use std::path::PathBuf;

use inquire::{Confirm, CustomType, Text};
use rand::Rng;
use toml::Value;

use crate::{
    autocomplete,
    logs::write_to_log,
    password,
    storage::{self, write_to_file},
};

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
    write_to_log(dir, "Listed all stored passwords.")?;
    Ok(())
}

pub fn get(
    dir: &PathBuf,
    key: &Option<String>,
    master_password: &Option<String>,
) -> Result<(), String> {
    let passwords = storage::get_passwords_from_file(dir)?;

    let key = match key {
        Some(k) => k,
        None => &{
            let autocomplete = autocomplete::KeyCompleter {
                keys: passwords
                    .keys()
                    .cloned()
                    .filter(|k| k != storage::VERSION_KEY)
                    .collect(),
            };
            let key_input = Text::new("Key: ").with_autocomplete(autocomplete).prompt();
            match key_input {
                Err(e) => {
                    return Err(format!("Could not read key input.\n\n{}", e));
                }
                Ok(k) => k,
            }
        },
    };

    let master_password = match master_password {
        Some(mp) => mp,
        None => &{
            let master_password_input = Text::new("Master password:").prompt();
            match master_password_input {
                Err(e) => {
                    return Err(format!("Could not read master password input.\n\n{}", e));
                }
                Ok(mp) => mp,
            }
        },
    };

    let password = match passwords.get(key) {
        Some(value) => {
            let value = match value {
                Value::String(s) => s,
                _ => {
                    return Err(format!("Password for key '{}' is not a valid string.", key));
                }
            };

            let value = match value.strip_prefix('"') {
                Some(v) => v,
                None => value,
            };
            let value = match value.strip_suffix('"') {
                Some(v) => v,
                None => value,
            };

            value
        }
        None => return Err(format!("No password found for key '{}'.", key)),
    };

    let decrypted_password = password::decrypt(password, &master_password);
    let decrypted_password = match decrypted_password {
        Err(e) => {
            return Err(format!(
                "Could not decrypt password for key '{}'.\n\n{}",
                key, e
            ));
        }
        Ok(dp) => dp,
    };

    write_to_log(dir, format!("Retrieved password for key {}.", key).as_str())?;
    println!("Password for '{}':\n{}", key, decrypted_password);
    Ok(())
}

pub fn set(
    dir: &PathBuf,
    key: &Option<String>,
    value: &Option<String>,
    master_password: &Option<String>,
) -> Result<(), String> {
    let mut passwords = storage::get_passwords_from_file(dir)?;

    let key = match key {
        Some(k) => k,
        None => &{
            let autocomplete = autocomplete::KeyCompleter {
                keys: passwords
                    .keys()
                    .cloned()
                    .filter(|k| k != storage::VERSION_KEY)
                    .collect(),
            };
            let key_input = Text::new("Key: ").with_autocomplete(autocomplete).prompt();
            match key_input {
                Err(e) => {
                    return Err(format!("Could not read key input.\n\n{}", e));
                }
                Ok(k) => k,
            }
        },
    };

    let value = match value {
        Some(v) => v,
        None => &{
            let value_input = Text::new("Password: ").prompt();
            match value_input {
                Err(e) => {
                    return Err(format!("Could not read password input.\n\n{}", e));
                }
                Ok(v) => v,
            }
        },
    };

    let master_password = match master_password {
        Some(mp) => mp,
        None => &{
            let master_password_input = Text::new("Master password:").prompt();
            match master_password_input {
                Err(e) => {
                    return Err(format!("Could not read master password input.\n\n{}", e));
                }
                Ok(mp) => mp,
            }
        },
    };

    let encrypted_password = password::encrypt(value, &master_password)?;
    passwords.insert(key.to_string(), Value::String(encrypted_password));

    let write = storage::write_to_file(&dir, &passwords);
    match write {
        Err(e) => Err(format!(
            "Could not write updated passwords to file.\n\n{}",
            e
        )),
        Ok(_) => {
            write_to_log(dir, format!("Set password for key {}.", key).as_str())?;
            println!("Password for '{}' set successfully.", key);
            Ok(())
        }
    }
}

pub fn generate(
    dir: &PathBuf,
    key: &Option<String>,
    special_chars: Option<bool>,
    length: Option<usize>,
    master_password: &Option<String>,
) -> Result<(), String> {
    let mut passwords = storage::get_passwords_from_file(dir)?;

    let key = match key {
        Some(k) => k,
        None => &{
            let autocomplete = autocomplete::KeyCompleter {
                keys: passwords
                    .keys()
                    .cloned()
                    .filter(|k| k != storage::VERSION_KEY)
                    .collect(),
            };
            let key_input = Text::new("Key: ").with_autocomplete(autocomplete).prompt();
            match key_input {
                Err(e) => {
                    return Err(format!("Could not read key input.\n\n{}", e));
                }
                Ok(k) => k,
            }
        },
    };

    let special_chars = match special_chars {
        Some(s) => s,
        None => {
            let special_chars_input = Confirm::new("Include special characters?")
                .with_default(true)
                .prompt();
            match special_chars_input {
                Err(e) => {
                    return Err(format!("Could not read special characters input.\n\n{}", e));
                }
                Ok(s) => s,
            }
        }
    };

    let length = match length {
        Some(l) => l,
        None => {
            let length_input = CustomType::<usize>::new("Password length: ")
                .with_default(32)
                .with_error_message("Please enter a valid number.")
                .with_help_message("Recommended length is at least 12.")
                .prompt();
            match length_input {
                Err(e) => {
                    return Err(format!("Could not read password length input.\n\n{}", e));
                }
                Ok(l) => l,
            }
        }
    };

    let master_password = match master_password {
        Some(mp) => mp,
        None => &{
            let master_password_input = Text::new("Master password:").prompt();
            match master_password_input {
                Err(e) => {
                    return Err(format!("Could not read master password input.\n\n{}", e));
                }
                Ok(mp) => mp,
            }
        },
    };

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

    let encrypted_password = password::encrypt(&password, &master_password)?;
    passwords.insert(key.to_string(), Value::String(encrypted_password));
    write_to_file(dir, &passwords)?;

    write_to_log(dir, format!("Generated password for key {}.", key).as_str())?;
    println!("Generated and saved password for '{}':\n{}", key, password);
    Ok(())
}
