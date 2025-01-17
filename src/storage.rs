use std::{fs, path::Path};
use crate::models::Item;

const FILE_PATH: &str = "storage.json";

pub fn read_storage() -> Vec<Item> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).expect("Failed to read storage file");
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

pub fn write_storage(items: &[Item]) {
    let data = serde_json::to_string(items).expect("Failed to serialize items");
    fs::write(FILE_PATH, data).expect("Failed to write to storage file");
}
