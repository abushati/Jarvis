use std::sync::Arc;
use std::{env, fs, path::PathBuf,path::Path, str::FromStr};
extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use chrono::{DateTime, Utc};
use std::env::args;
use std::collections::HashMap;
use std::fs::OpenOptions;
use core::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
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


// struct FileCleaner {
//     file_manager: FileManager,
//     max_file_age: u64,
//     // to_delete_queue: PathBuf
// }


struct CliAction {
    cmd: Box<dyn CLICommand>,
}
#[derive(Debug,Serialize, Deserialize, Default)]
struct FileManager {
    excluded_files: Vec<File>,
    excluded_directories: Vec<Directory>,
    included_directories: Vec<Directory>,
}

const default_file_manager_template:&str = r###"{
    "excluded_files": [],
    "excluded_directories": [],
    "included_directories": []
}"###;

impl FileManager {
    fn save (&self) {
        let file_name = "FileMananger.json";
        // let current_dir = String::new(env::current_dir().unwrap());
        let mut file = fs::File::create(&file_name).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        let _ = file.write_all(json
            .as_bytes())
            .unwrap();
    }

    fn load (&self) -> FileManager {
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

    fn reset(&self) -> FileManager{
        fs::remove_file("FileMananger.json").unwrap();
        let manager = self.load();
        manager.save();
        return manager;
    }

    fn add(mut self, section: &file_manager_section, path: &str ) -> Self{
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
                }
            }
        }
        self.save();
        // println!("{:?}", typed);
        self
    }

    fn remove(mut self, section: &file_manager_section, path: &str ) -> Self
    {
        return self
    }
    // fn remove(mut self, section: file_manager_section, path: &str ) -> Self{
    //     match section {
    //         file_manager_section::EXCLUDE_DIR => {
    //             if let Some(pos) = vec.iter().position(|x| *x == needle) {
    //                 vec.remove(pos);
    //             }
    //             self.excluded_directories.remove(walk_directory(PathBuf::from_str(path).unwrap()));
    //         },
    //         file_manager_section::INCLUDE_DIR => {
    //             self.included_directories.remove(walk_directory(PathBuf::from_str(path).unwrap()));
    //         },
    //         file_manager_section::EXCLUDE_FILE => {
    //             self.excluded_files.(PathBuf::from_str(path).unwrap());
    //         }
            
    //     }
    //     self.save();
    //     println!("{:?}", typed);
    //     self
    // }

    }


// impl FileCleaner{
//     fn clean(&self){
//         let dirs_to_clean = &self.file_manager.included_directories;
//         for dir in dirs_to_clean{
//             self.clean_dir_files(dir);
//         }
//     }
//     fn check_excluded (&self, dir: &Directory, let_dir_to_check: &Vec<Directory> ) -> bool{
//         for excluded_dir in let_dir_to_check {
//             if dir.path == excluded_dir.path{
//                 return true
//             }
//             if !excluded_dir.child_directories.is_empty(){            
//                 if self.check_excluded(dir, &excluded_dir.child_directories){
//                     return true
//                 }
    
//             }
//         }
//         false
//     }
//     fn check_dir_in_excluded(&self, dir: &Directory) -> bool {
//         return self.check_excluded(&dir, &self.file_manager.excluded_directories)
//     }
//     fn clean_dir_files(&self, dir: &Directory) {
//         if self.check_dir_in_excluded(dir){
//             println!("{:?} dir is excluded", dir.path);
//             return
//         }
        
//         let mut to_delete_queue: Vec<String>  = vec![];
//         let mut file = OpenOptions::new()
//         .read(true)
//         .open("to_delete_queue.txt")
//         .unwrap();

//         let mut buf = String::new();
//         file.read_to_string(&mut buf).unwrap();
//         let list :Vec<&str> = buf.lines().collect();
//         println!("from buff{:?}", list);

//         let mut file = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .append(true)
//         .open("to_delete_queue.txt")
//         .unwrap();

//         for file in &dir.files {
//             let now: DateTime<Utc> = file.last_accessed.into();
//             let file_path = file.path.to_str().unwrap();
//             if !list.contains(&file_path) && self.should_delete_file(file){
//                 to_delete_queue.push(String::from(file_path));
//             }
//         }

//         for path in to_delete_queue {
//             file.write_all(path.as_bytes()).expect("write failed");
//             file.write_all("\n".as_bytes());
//         }

//         for dir in &dir.child_directories{
//             self.clean_dir_files(dir)
//         }
//     }
    
//     fn should_delete_file(&self, file: &File) -> bool {
//         if file.last_accessed.elapsed().unwrap().as_secs() > self.max_file_age{
//             return true
//         }
//         return false
//         // println!("{:?} file name to delete",file.last_accessed.elapsed().unwrap());
//         // return true
//     }
    
// }


trait  CLICommand {
    fn run(&self){}
}
enum primary_cmds {
    MANAGER,
    //  CONFIG,
    //   CLEAN
    }
impl FromStr for primary_cmds {

    type Err = ();

    fn from_str(input: &str) -> Result<primary_cmds, Self::Err> {
        let  input = input.to_uppercase();
        match input.as_str() {
            "MANAGER"  => Ok(primary_cmds::MANAGER),
            // "CONFIG"  => Ok(primary_cmds::CONFIG),
            // "CLEAN"  => Ok(primary_cmds::CLEAN),
            _      => Err(()),
        }
    }
}


#[derive(Debug)]
enum file_manager_section {
    EXCLUDE_DIR,
    INCLUDE_DIR,
    EXCLUDE_FILE,
}

impl FromStr for file_manager_section {

    type Err = ();

    fn from_str(input: &str) -> Result<file_manager_section, Self::Err> {
        let  input = input.to_uppercase();
        match input.as_str() {
            "EXCLUDE_DIR"  => Ok(file_manager_section::EXCLUDE_DIR),
            "INCLUDE_DIR"  => Ok(file_manager_section::INCLUDE_DIR),
            "EXCLUDE_FILE"  => Ok(file_manager_section::EXCLUDE_FILE),
            _      => Err(()),
        }
    }
}
#[derive(Debug,PartialEq)]
enum manager_actions {
    ADD,
    REMOVE,
    RESET
    }

impl FromStr for manager_actions {
    type Err = ();
    fn from_str(input: &str) -> Result<manager_actions, Self::Err> {
        let  input = input.to_uppercase();
        match input.as_str() {
            "ADD"  => Ok(manager_actions::ADD),
            "REMOVE"  => Ok(manager_actions::REMOVE),
            "RESET"  => Ok(manager_actions::RESET),
            _      => Err(()),
        }
    }
}  

struct manager_cmd{
    manager_action: manager_actions,
    sub_action: Option<file_manager_section>,
    value: Option<String>
}
impl CLICommand for manager_cmd {
    fn run(&self) {
        println!("{:?}",&self.manager_action);
        let file_manager = FileManager::default().load();
        match self.manager_action {
            manager_actions::ADD => {
                file_manager.add(&self.sub_action.as_ref().ok_or("no").unwrap(),self.value.as_ref().ok_or("no").unwrap().as_str());
            },
            manager_actions::REMOVE => {
                file_manager.remove(&self.sub_action.as_ref().ok_or("no").unwrap(),self.value.as_ref().ok_or("no").unwrap().as_str());
            },
            manager_actions::RESET => {
                file_manager.reset();
            }
        }
    }
}

fn parse_args() -> Result<CliAction,String> {
    // {"manager":{"add":["action","type"],
    //             "remove":["action","type"],
    //             "reset": []
    //              },
    //"config":["action","key","value"]
    //         
    // "clean":{}}
    let args:Vec<String> = args().collect();
    if !(args.len() > 1){
        return Err(String::from_str("invalid args len").unwrap());
    }
    
    let primary_cmd = args.get(1).unwrap();
    match primary_cmds::from_str(&primary_cmd){
        Ok(act) => {
            match act {
                primary_cmds::MANAGER => {
                    let manager_action;
                    let manager_section;
                    let value;
                    if args.get(2).is_none(){
                        return Err(format!("No sub action provided, valid args: {:?}" , "adf"));
                    }
                    match manager_actions::from_str(args.get(2).unwrap()){
                        Ok(action) => {
                            manager_action = action;
                        }
                        Err(()) => {
                            return Err(format!("Invalid manager action" ));
                        }
                    }
                    
                    if manager_action == manager_actions::RESET {
                        let cmd = manager_cmd{manager_action:manager_action, sub_action: None, value: None};
                        return Ok(CliAction{cmd:Box::new(cmd)});
                    }

                    let section = args.get(3).unwrap();
                    match file_manager_section::from_str(&section.as_str()) {
                        Ok(section) => {
                            manager_section = section
                        }
                        Err(()) => {
                            return Err(format!("Invalid subaction {}, valid args: {:?}" ,"adf", "adf"));
                        }
                    }
                    
                    let path = args.get(4);
                    if !path.is_none(){
                        value = path.unwrap();

                    } else {
                        return Err(format!("path not provided"));
                    }

                    let cmd = manager_cmd{manager_action:manager_action,sub_action:Some(manager_section),value:Some(value.to_string())};
                    Ok(CliAction{cmd:Box::new(cmd)})
                },
                // primary_cmds::CONFIG => {return Err()},
                // primary_cmds::CLEAN => {return Err()}

            }

        }
        Err(()) => {return Err(format!("invalid primary arg {}, valid args: {:?}", "primary_cmd", "valid_primary_cmds"));}
    } 
}

fn main() {
    let db = parse_args();
    match db {
        Ok(action) => {
            let cmd = action.cmd;
            cmd.run()
            // action.run_action();
        },
        Err(error) =>{
            println!("{}", error)
        }
        
    }
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