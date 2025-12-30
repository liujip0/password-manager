use toml::{Table, Value};

use crate::storage;

pub fn encrypt(password: &str, master_password: &str) -> Result<String, String> {
    if master_password.len() == 0 {
        return Ok(password.to_string());
    }

    let password = password.as_bytes();
    let master_password = master_password.as_bytes();

    let encrypted_password: Vec<u8> = password
        .iter()
        .enumerate()
        .map(|i| i.1 ^ master_password[i.0 % master_password.len()])
        .collect();

    let encrypted_password = String::from_utf8(encrypted_password);
    match encrypted_password {
        Err(e) => Err(format!("Could not encrypt password.\n\n{}", e)),
        Ok(ep) => Ok(ep),
    }
}

pub fn decrypt(encrypted_password: &str, master_password: &str) -> Result<String, String> {
    if master_password.len() == 0 {
        return Ok(encrypted_password.to_string());
    }

    let encrypted_password = encrypted_password.as_bytes();
    let master_password = master_password.as_bytes();

    let decrypted_password: Vec<u8> = encrypted_password
        .iter()
        .enumerate()
        .map(|i| i.1 ^ master_password[i.0 % master_password.len()])
        .collect();

    let decrypted_password = String::from_utf8(decrypted_password);
    match decrypted_password {
        Err(e) => Err(format!("Could not decrypt password.\n\n{}", e)),
        Ok(dp) => Ok(dp),
    }
}

pub fn bulk_decrypt(passwords: Table, master_password: &str) -> Result<Table, String> {
    if master_password.len() == 0 {
        return Ok(passwords);
    }

    let mut passwords_entries = Table::new();

    for (k, v) in passwords.iter() {
        if k == storage::VERSION_KEY {
            passwords_entries.insert(k.clone(), v.clone());
            continue;
        }

        let value = match v {
            Value::String(s) => s,
            _ => "",
        };

        let value = match value.strip_prefix('"') {
            Some(v) => v,
            None => value,
        };
        let value = match value.strip_suffix('"') {
            Some(v) => v,
            None => value,
        };

        let decrypted_value = decrypt(value, master_password)?;
        passwords_entries.insert(k.clone(), Value::String(decrypted_value));
    }

    Ok(passwords_entries)
}
