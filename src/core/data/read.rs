use crate::core::database;
use std::path::Path;

pub fn read_from_file(mut db: database::Database) -> database::Database {
    let homedir = std::env::var("HOME").unwrap();
    let filepath = Path::new(homedir.as_str())
        .join(".config")
        .join("timetrustlings")
        .join("data.yml");

    match std::fs::read_to_string(filepath) {
        Ok(data) => {
            db = serde_yaml::from_str(data.as_str()).unwrap();
        }
        Err(_) => {}
    };
    return db;
}
