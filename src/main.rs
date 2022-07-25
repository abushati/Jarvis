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
    files: Option<Vec<File>>,
    child_directories: Option<Vec<Directory>>,
}

#[derive(Debug,Serialize, Deserialize)]
struct FileManager {
    excluded_files: Option<Vec<File>>,
    excluded_directories: Option<Vec<Directory>>,
    included_directories: Option<Vec<Directory>>,
}

impl Save for FileManager {
    fn save (&self) {
        let file_name = "FileMananger.json";
        // let current_dir = String::new(env::current_dir().unwrap());
        let mut file = fs::File::create(&file_name).unwrap();
        let json = serde_json::to_string(&self).unwrap();
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
        let okay = file.write_all(json.as_bytes()).unwrap();
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
    let dire =  current_dir.unwrap();
    let files = walk_directory(dire);
    for file in files {
        println!("{:?}", file.file_name);
        file.save();
        break;
    }
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