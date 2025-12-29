pub fn encrypt(password: &str, master_password: &str) -> Result<String, String> {
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
