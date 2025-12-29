use std::path::PathBuf;

use crate::storage;

pub fn list(dir: &PathBuf) -> Result<(), String> {
    let passwords = storage::get_passwords_from_file(dir)?;

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
