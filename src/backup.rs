use std::{fs, path::PathBuf};

use clap::ValueEnum;
use inquire::{Confirm, Select, Text};
use toml::{Table, Value};

use crate::{ExportType, password, storage};

pub fn export_to_file(
    dir: &PathBuf,
    export_path: &Option<String>,
    export_type: &Option<ExportType>,
    master_password: &Option<String>,
) -> Result<(), String> {
    let mut export_path = match export_path {
        Some(path) => PathBuf::from(path),
        None => {
            let export_path_input = Text::new("File path:")
                .with_help_message("File path to save passwords to")
                .prompt();
            match export_path_input {
                Err(e) => {
                    return Err(format!("Could not read file path input.\n\n{}", e));
                }
                Ok(path) => PathBuf::from(path),
            }
        }
    };

    let mut export_type = export_type;
    let extension = match export_path.extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or(""),
    };
    match extension {
        "json" => export_type = &Some(ExportType::Json),
        "csv" => export_type = &Some(ExportType::Csv),
        "toml" => export_type = &Some(ExportType::Toml),
        _ => {}
    };

    let export_type = match export_type {
        Some(et) => et,
        None => &{
            let options = ExportType::value_variants().to_vec();
            let export_type_input = Select::new("Export file type: ", options).prompt();
            match export_type_input {
                Err(e) => {
                    return Err(format!("Could not read export file type input.\n\n{}", e));
                }
                Ok(et) => {
                    export_path.add_extension(et.to_string());
                    et
                }
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

    let passwords = storage::get_passwords_from_file(dir)?;
    let decrypted_passwords = password::bulk_decrypt(passwords, master_password)?;

    let contents: String = match export_type {
        ExportType::Json => match serde_json::to_string_pretty(&decrypted_passwords) {
            Err(e) => {
                return Err(format!(
                    "Could not serialize passwords to JSON format\n\n{}",
                    e
                ));
            }
            Ok(contents) => contents,
        },
        ExportType::Csv => {
            let mut wtr = csv::Writer::from_writer(Vec::new());
            for pwd in decrypted_passwords {
                match wtr.serialize(pwd) {
                    Err(e) => {
                        return Err(format!(
                            "Could not serialize passwords to CSV format\n\n{}",
                            e
                        ));
                    }
                    Ok(_) => {}
                };
            }
            match wtr.flush() {
                Err(e) => {
                    return Err(format!(
                        "Could not serialize passwords to CSV format\n\n{}",
                        e
                    ));
                }
                Ok(_) => {}
            };

            let bytes = wtr.into_inner();
            let bytes = match bytes {
                Err(e) => {
                    return Err(format!(
                        "Could not serialize passwords to CSV format\n\n{}",
                        e
                    ));
                }
                Ok(b) => b,
            };

            let csv_string = match String::from_utf8(bytes) {
                Err(e) => {
                    return Err(format!("Could not convert CSV bytes to string\n\n{}", e));
                }
                Ok(s) => s,
            };
            csv_string
        }
        ExportType::Toml => match toml::to_string_pretty(&decrypted_passwords) {
            Err(e) => {
                return Err(format!(
                    "Could not serialize passwords to TOML format\n\n{}",
                    e
                ));
            }
            Ok(contents) => contents,
        },
    };

    let write = fs::write(&export_path, contents);
    match write {
        Err(e) => {
            return Err(format!(
                "Could not write passwords to file at {}\n\n{}",
                &export_path.display(),
                e
            ));
        }
        Ok(_) => {
            println!(
                "Passwords successfully exported to {}",
                &export_path.display()
            );
            Ok(())
        }
    }
}

pub fn import_from_file(
    dir: &PathBuf,
    import_path: &Option<String>,
    master_password: &Option<String>,
    overwrite: Option<bool>,
) -> Result<(), String> {
    let import_path = match import_path {
        Some(path) => PathBuf::from(path),
        None => {
            let import_path_input = Text::new("File path:")
                .with_help_message("File path to import passwords from")
                .prompt();
            match import_path_input {
                Err(e) => {
                    return Err(format!("Could not read file path input.\n\n{}", e));
                }
                Ok(path) => PathBuf::from(path),
            }
        }
    };

    let overwrite = match overwrite {
        Some(o) => o,
        None => {
            let overwrite_input = Confirm::new("Overwrite existing passwords?")
                .with_default(false)
                .prompt();
            match overwrite_input {
                Err(e) => {
                    return Err(format!(
                        "Could not read overwrite existing passwords input.\n\n{}",
                        e
                    ));
                }
                Ok(o) => o,
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

    let file = fs::read_to_string(&import_path);
    let file = match file {
        Err(e) => {
            return Err(format!(
                "Could not read passwords file at {}\n\n{}",
                &import_path.display(),
                e
            ));
        }
        Ok(contents) => contents,
    };

    let extension = match import_path.extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap_or("[unknown]"),
    };

    let imported_passwords: Table;
    match extension {
        "json" => {
            let passwords = serde_json::from_str::<Table>(&file);
            let passwords = match passwords {
                Err(e) => {
                    return Err(format!(
                        "Could not parse passwords file at {} as JSON format\n\n{}",
                        &import_path.display(),
                        e
                    ));
                }
                Ok(p) => p,
            };

            imported_passwords = passwords;
        }
        "csv" => {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file.as_bytes());

            let mut passwords = Table::new();
            for row in rdr.deserialize() {
                let record: Vec<String> = match row {
                    Err(e) => {
                        return Err(format!(
                            "Could not parse passwords file at {} as CSV format\n\n{}",
                            &import_path.display(),
                            e
                        ));
                    }
                    Ok(rec) => rec,
                };

                passwords.insert(record[0].clone(), Value::String(record[1].clone()));
            }

            imported_passwords = passwords;
        }
        "toml" => {
            let passwords = toml::from_str::<Table>(&file);
            let passwords = match passwords {
                Err(e) => {
                    return Err(format!(
                        "Could not parse passwords file at {} as TOML format\n\n{}",
                        &import_path.display(),
                        e
                    ));
                }
                Ok(p) => p,
            };

            imported_passwords = passwords;
        }
        _ => {
            return Err(format!("Unsupported import file type: {}", extension));
        }
    };

    if imported_passwords.get(storage::VERSION_KEY)
        != Some(&Value::String(env!("CARGO_PKG_VERSION").to_string()))
    {
        let file_version = imported_passwords.get(storage::VERSION_KEY);
        let file_version = match file_version {
            Some(Value::String(v)) => v,
            _ => "[unknown version]",
        };

        return Err(format!(
            "Passwords file version ({}) does not match application version ({}). Import aborted.",
            file_version,
            env!("CARGO_PKG_VERSION")
        ));
    };

    let mut current_passwords = storage::get_passwords_from_file(dir)?;

    for (k, v) in imported_passwords.iter() {
        if k == storage::VERSION_KEY {
            continue;
        }

        let v = match v {
            Value::String(s) => s,
            _ => {
                return Err(format!(
                    "Invalid password value for key {} in imported file. Expected string.",
                    k
                ));
            }
        };
        let encrypted_password = password::encrypt(v, master_password)?;

        if current_passwords.contains_key(k) {
            if overwrite {
                let old_password =
                    current_passwords.insert(k.clone(), Value::String(encrypted_password));
                let old_password = match old_password {
                    Some(Value::String(op)) => op,
                    _ => "[non-string value]".to_string(),
                };
                println!(
                    "Password for key {} overwritten (old value: {:?}).",
                    k,
                    password::decrypt(&old_password, master_password)?
                );
            } else {
                println!("Password already exists for key {}. Skipping...", k);
                continue;
            }
        } else {
            current_passwords.insert(k.clone(), Value::String(encrypted_password));
            println!("Password for key {} imported.", k);
        }
    }

    storage::write_to_file(&dir, &current_passwords)?;

    println!("Import finished successfully.");
    Ok(())
}
