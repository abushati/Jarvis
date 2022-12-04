extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize};
use core::fmt::Debug;
use std::{fs,path::PathBuf,str::FromStr};
use std::io::prelude::*;


#[derive(Debug,Serialize, Deserialize, Default, Clone)]
pub struct FileManager {
    pub excluded_files: Vec<File>,
    pub excluded_directories: Vec<Directory>,
    pub included_directories: Vec<Directory>,
}

pub enum file_manager_section {
    EXCLUDE_DIR,
    INCLUDE_DIR,
    EXCLUDE_FILE,
}


impl FileManager {
    pub fn save (&self) {
        let file_name = "FileMananger.json";
        // let current_dir = String::new(env::current_dir().unwrap());
        let mut file = fs::File::create(&file_name).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        let _ = file.write_all(json
            .as_bytes())
            .unwrap();
    }

    pub fn load (&self) -> FileManager {
        let file_name = "FileMananger.json";
        let file_string; 
        let mut file = fs::read_to_string(&file_name);
        if file.is_err() {
            let default_template = r###"{
                "excluded_files": [],
                "excluded_directories": [],
                "included_directories": []
            }"###.to_string();
            file_string = default_template.to_string();
        }
        else {
            file_string = file.unwrap();
            
        }
        let manager: Result<FileManager, serde_json::Error> = serde_json::from_str(&file_string);
        if manager.is_err(){
            return self.reset();
        }
        return manager.unwrap()
    }

    pub fn reset(&self) -> FileManager{
        fs::remove_file("FileMananger.json").unwrap();
        let manager = self.load();
        manager.save();
        return manager;
    }
    //Need a way to detect duplicates and stop
    pub fn add(mut self, section: &file_manager_section, path: &str ) -> Self{
        match section {
            file_manager_section::EXCLUDE_DIR => {
                self.excluded_directories.push(walk_directory(PathBuf::from_str(path).unwrap()));
            },
            file_manager_section::INCLUDE_DIR => {
                self.included_directories.push(walk_directory(PathBuf::from_str(path).unwrap()));
            },
            file_manager_section::EXCLUDE_FILE => {
                let pathBuff = PathBuf::from_str(path).unwrap();
                let file_name = pathBuff.file_name().unwrap().to_str().unwrap().to_string();
                let file_path = pathBuff;

                let metadata = fs::metadata(&path).unwrap();
                
                if metadata.is_file() {
                    let file = File {
                        path: file_path,
                        file_name: file_name,
                        last_accessed: metadata.accessed().unwrap(),
                        last_modified: metadata.modified().unwrap(),
                        created: metadata.created().unwrap(),
                        };

                    self.excluded_files.push(file);
                } else {
                    println!("input is not a file")
                }
            }
        }
        self.save();
        // println!("{:?}", typed);
        self
    }

    pub fn remove(mut self, section: &file_manager_section, path: &str ) -> Self{
        match section {
            file_manager_section::EXCLUDE_DIR => {
                if let Some(pos) = self.excluded_directories.iter().position(|x| * &x.path.to_str().ok_or("not string").unwrap() == path) {
                    self.excluded_directories.remove(pos);
                } else {
                    println!("Couldn't remove from excluded directories")
                }   

            },
            file_manager_section::INCLUDE_DIR => {
                if let Some(pos) = self.included_directories.iter().position(|x| * &x.path.to_str().ok_or("not string").unwrap() == path) {
                    self.included_directories.remove(pos);
                } else {
                    println!("Couldn't remove from included directories")
                }   
            },
            file_manager_section::EXCLUDE_FILE => {
                if let Some(pos) = self.excluded_files.iter().position(|x| * &x.path.to_str().ok_or("not string").unwrap() == path) {
                    self.excluded_files.remove(pos);
                } else {
                    println!("Couldn't remove from excluded file")
                }   
            }
            
        }
        self.save();
        return self
    }

    }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub path: std::path::PathBuf,
    pub file_name: String,
    pub last_accessed: std::time::SystemTime,
    pub last_modified:std::time::SystemTime,
    pub created: std::time::SystemTime,
}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct Directory {
    pub path: std::path::PathBuf,
    pub files: Vec<File>,
    pub child_directories: Vec<Directory>,
}

fn walk_directory(directory:std::path::PathBuf) -> Directory {
    let mut current_dir_files: Vec<File> = Vec::new();
    let mut current_dir_dir: Vec<Directory> = Vec::new();
    let directs = fs::read_dir(&directory).unwrap();
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
        else if metadata.is_dir() {
            current_dir_dir.push(walk_directory(file_path))
        }
    }
    return Directory {path: directory, files: current_dir_files, child_directories: current_dir_dir};
}
