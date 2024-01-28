use crate::core::database;
use std::{
    fs::{self, File},
    path::Path,
};

pub fn write_to_file(db: &database::Database) {
    let homedir = std::env::var("HOME").unwrap();
    let filepath = Path::new(homedir.as_str())
        .join(".config")
        .join("timetrustlings")
        .join("data.yml");

    let prefix = filepath.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let _ = File::create(&filepath);
    let yaml_data = serde_yaml::to_string(&db).unwrap();
    fs::write(filepath, yaml_data).expect("Failed to write to system");
}
