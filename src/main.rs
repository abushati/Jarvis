use std::{env, fs};
extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

trait Save {
    fn save(&self);
    fn load(&self);
   }


#[derive(Debug,Serialize, Deserialize)]
struct File {
    path: std::path::PathBuf,
    file_name: String,
    last_accessed: std::time::SystemTime,
    last_modified:std::time::SystemTime,
    created: std::time::SystemTime,
}

#[derive(Debug,Serialize, Deserialize)]
struct Directory {
    path: std::path::PathBuf,
    files: Vec<File>,
    child_directories: Vec<Directory>,
}

#[derive(Debug,Serialize, Deserialize, Default)]
struct FileManager {
    excluded_files: Vec<File>,
    excluded_directories: Vec<Directory>,
    included_directories: Vec<Directory>,
}


impl Save for FileManager {
    fn save (&self) {
        let file_name = "FileMananger.json";
        // let current_dir = String::new(env::current_dir().unwrap());
        let mut file = fs::File::create(&file_name).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        // println!("{:?}", json);
        let _ = file.write_all(json
            .as_bytes())
            .unwrap();
        println!("{:?}", json);
    }

    fn load (&self) {
    }
}

impl Save for File {
    fn save (&self) {
        let file_name = "File.json";
        // let current_dir = String::new(env::current_dir().unwrap());
        let mut file = fs::File::create(&file_name).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        let _ = file.write_all(json
            .as_bytes())
            .unwrap();
        println!("{:?}", json);
    }

    fn load (&self) {
    }
}


fn main() {
    let current_dir = env::current_dir();
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    let mut file_manager = FileManager::default();
    println!("{:?}", file_manager);
    let excluded_files = &file_manager.excluded_files;
    let dire =  current_dir.unwrap();
    let files = walk_directory(dire);
    for file in files {
        // println!("{:?}", file.file_name);
        
        file_manager.excluded_files.push(file);
        
        // break;
    }
    // file_manager.excluded_files = <Option excluded_files>;
    println!("{:?}", file_manager);
    file_manager.save();
}

fn walk_directory(directory:std::path::PathBuf) -> Vec<File> {
    let mut current_dir_files: Vec<File> = Vec::new();
    let directs = fs::read_dir(directory).unwrap();
    for  entry in directs {
        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap();
        let file_path = entry.path();
        
        let metadata = fs::metadata(&file_path).unwrap();
        if metadata.is_file() {
            let file = File {
                path: file_path,
                file_name: file_name,
                last_accessed: metadata.accessed().unwrap(),
                last_modified: metadata.modified().unwrap(),
                created: metadata.created().unwrap(),
                };
            current_dir_files.push(file);
        }
    }
    current_dir_files
}