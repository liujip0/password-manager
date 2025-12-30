use std::{fs, io::Write, path::PathBuf};

use chrono::{Local, SecondsFormat};

const LOGS_FILE: &str = "liujip0-password-manager.log";

pub fn write_to_log(dir: &PathBuf, message: &str) -> Result<(), String> {
    let file_path = dir.join(LOGS_FILE);
    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path);
    let mut file = match file {
        Err(e) => {
            return Err(format!(
                "Could not open logs file at {}\n\n{}",
                &file_path.display(),
                e
            ));
        }
        Ok(f) => f,
    };

    let date_time = Local::now();
    let log_entry = format!(
        "{} - {}\n",
        date_time.to_rfc3339_opts(SecondsFormat::Millis, true),
        message
    );

    let write = file.write_all(log_entry.as_bytes());
    match write {
        Err(e) => {
            return Err(format!(
                "Could not write to logs file at {}\n\n{}",
                file_path.display(),
                e
            ));
        }
        Ok(_) => {}
    };

    let sync = file.sync_all();
    match sync {
        Err(e) => {
            return Err(format!(
                "Could not sync logs file at {}\n\n{}",
                file_path.display(),
                e
            ));
        }
        Ok(_) => {}
    };

    Ok(())
}
