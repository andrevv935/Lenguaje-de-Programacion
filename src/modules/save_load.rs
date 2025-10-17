use std::fs;
use std::path::Path;
use serde_json;

use crate::modules::Tasks;

const DATA_DIR: &str = "data";
const DATA_FILE: &str = "data/tasks.json";

pub fn load_data(tasks_array: &mut Vec<Tasks>){
    if !Path::new(DATA_FILE).exists() {
        // If the file doesn't exist, ensure directory exists and create an empty file
        let _ = fs::create_dir_all(DATA_DIR);
        let _ = fs::write(DATA_FILE, "[]");
    }

    let data = fs::read_to_string(DATA_FILE)
        .expect("Unable to read file");
    if data.trim().is_empty() {
        return;
    }
    let loaded_tasks: Vec<Tasks> = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");
    tasks_array.extend(loaded_tasks);
}

pub fn save_data(tasks_array: &Vec<Tasks>){
    if !Path::new(DATA_DIR).exists() {
        fs::create_dir_all(DATA_DIR).expect("Unable to create data directory");
    }
    let json = serde_json::to_string_pretty(tasks_array).expect("Failed to serialize tasks");
    fs::write(DATA_FILE, json).expect("Unable to write file");
}