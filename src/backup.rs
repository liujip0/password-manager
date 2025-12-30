use std::{fs, path::PathBuf};

use clap::ValueEnum;
use inquire::{Select, Text};

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
