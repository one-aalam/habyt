extern crate dirs;
use crate::store::{ HabitStore, HabitLogStore };

use std::fs;
use std::path::PathBuf;

const HABYT_DIR: &str = ".habyt";
const HABIT_STORE: &str = "habit_store.yaml";
const HABIT_LOG_STORE: &str = "habit_log_store.yaml";

pub struct HabitStoreFs {
    pub store: HabitStore,
    pub store_log: HabitLogStore,
    store_file: PathBuf,
    store_file_log: PathBuf
}

impl HabitStoreFs {
    pub fn new() -> HabitStoreFs {
        // Create the directory to maintain the entries
        let mut store_dir = dirs::home_dir().unwrap();
        store_dir.push(HABYT_DIR);
        if !store_dir.is_dir() {
            println!("Welcome to Habyt!\n");
            fs::create_dir(&store_dir).unwrap();
        }

        // Creat a file to log the habits
        let mut store_file = store_dir.clone();
        store_file.push(HABIT_STORE);

        let mut store_file_log = store_dir;
        store_file_log.push(HABIT_LOG_STORE);

        if !store_file.is_file() {
            fs::File::create(&store_file).unwrap();
            let content = serde_yaml::to_string(&HabitStore::new()).expect("Failed to serialize tickets");
            fs::write(&store_file, content).expect("Failed to write tickets to disk.");

            fs::File::create(&store_file_log).unwrap();
            let content = serde_yaml::to_string(&HabitLogStore::new()).expect("Failed to serialize tickets");
            fs::write(&store_file_log, content).expect("Failed to write tickets to disk.");

            println!(
                "Created {}. This file will list your currently tracked habits.",
                store_file.to_str().unwrap()
            );
        }

        HabitStoreFs {
            store: HabitStore::new(),
            store_log: HabitLogStore::new(),
            store_file,
            store_file_log
        }
    }

    pub fn load(&mut self) {
        self.store = match fs::read_to_string(&self.store_file) {
            Ok(data) => {
                serde_yaml::from_str(&data).expect("Failed to parse serialised data.")
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    HabitStore::new()
                }
                _ => panic!("Failed to read data."),
            },
        }
    }

    pub fn save(&self) {
        let content = serde_yaml::to_string(&self.store).expect("Failed to serialize tickets");
        fs::write(&self.store_file, content).expect("Failed to write tickets to disk.")
    }

    pub fn load_log(&mut self) {
        self.store_log = match fs::read_to_string(&self.store_file_log) {
            Ok(data) => {
                serde_yaml::from_str(&data).expect("Failed to parse serialised data.")
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    HabitLogStore::new()
                }
                _ => panic!("Failed to read data."),
            },
        }
    }

    pub fn save_log(&self) {
        let content = serde_yaml::to_string(&self.store_log).expect("Failed to serialize tickets");
        fs::write(&self.store_file_log, content).expect("Failed to write tickets to disk.")
    }
}
