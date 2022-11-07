use std::{env, fs, path::PathBuf,path::Path, str::FromStr};
extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use chrono::{DateTime, Utc};
use std::env::args;
use std::collections::HashMap;
use std::fs::OpenOptions;
trait Save {
    fn save(&self);
    fn load(&self) -> Self;
   }

#[derive(Debug)]
enum PathType {
    INCLUDE,
    EXCLUDE,
}

#[derive(Debug,PartialEq)]
enum Actions {
    ADD_DIR,
    REMOVE_DIR,
    CLEAN,
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

struct FileCleaner {
    file_manager: FileManager,
    max_file_age: u64,
    // to_delete_queue: PathBuf

}

#[derive(Debug)]
struct CliAction {
    action: Actions,
    args: Vec<String>
}

impl FileManager {
    fn add(mut self, path: &str,typed: &PathType) -> Self{
        match typed {
            PathType::EXCLUDE => {
                self.excluded_directories.push(walk_directory(PathBuf::from_str(path).unwrap()));
            },
            PathType::INCLUDE => {
                self.included_directories.push(walk_directory(PathBuf::from_str(path).unwrap()));
            }
        }
        self.save();
        println!("{:?}", typed);
        self
    }

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
        // println!("{:?}", json);
    }

    fn load (&self) -> FileManager {
        let file_name = "FileMananger.json";
        let file = fs::read_to_string(&file_name).unwrap();
        let manager: FileManager = serde_json::from_str(&file).unwrap();
        manager
    }
}


impl FileCleaner{
    fn clean(&self){
        let dirs_to_clean = &self.file_manager.included_directories;
        for dir in dirs_to_clean{
            self.clean_dir_files(dir);
        }
    }

    fn check_excluded (&self, dir:&Directory, let_dir_to_check: &Vec<Directory> ) -> bool{
        for excluded_dir in let_dir_to_check {
            if dir.path == excluded_dir.path{
                return true
            }
            if !excluded_dir.child_directories.is_empty(){            
                if self.check_excluded(dir, &excluded_dir.child_directories){
                    return true
                }
    
            }
        }
        false
    }

    fn check_dir_in_excluded(&self, dir: &Directory) -> bool {
        return self.check_excluded(&dir, &self.file_manager.excluded_directories)
    }

    fn clean_dir_files(&self, dir: &Directory) {
        if self.check_dir_in_excluded(dir){
            println!("{:?} dir is excluded", dir.path);
            return
        }
        
        let mut to_delete_queue: Vec<String>  = vec![];
        let mut file = OpenOptions::new()
        .read(true)
        .open("to_delete_queue.txt")
        .unwrap();

        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        let list :Vec<&str> = buf.lines().collect();
        println!("from buff{:?}", list);

        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("to_delete_queue.txt")
        .unwrap();

        for file in &dir.files {
            let now: DateTime<Utc> = file.last_accessed.into();
            let file_path = file.path.to_str().unwrap();
            if !list.contains(&file_path) && self.should_delete_file(file){
                to_delete_queue.push(String::from(file_path));
            }
            else  {
                // println!("{:?} file name NOT TO delete {:?}, {:?}",&file.file_name, now, &file.path)
            }
        }
        // println!("To queue {:?}", to_delete_queue);
        for path in to_delete_queue {
            file.write_all(path.as_bytes()).expect("write failed");
            file.write_all("\n".as_bytes());
        }

        for dir in &dir.child_directories{
            self.clean_dir_files(dir)
        }
    }
    
    fn should_delete_file(&self, file: &File) -> bool {
        if file.last_accessed.elapsed().unwrap().as_secs() > self.max_file_age{
            return true
        }
        return false
        // println!("{:?} file name to delete",file.last_accessed.elapsed().unwrap());
        // return true
    }
    
}

impl CliAction {
    fn run_action(self){
        let action = &self.action;
        
        if action == &Actions::ADD_DIR {
            self.add_dir()
        } else if action == &Actions::REMOVE_DIR {
            self.remove_dir()
        } else if action == &Actions::CLEAN {
            self.clean()
        }
    }
    fn add_dir(self) {
        let file_manager = FileManager::default().load();
        let new = HashMap::from([("include",PathType::INCLUDE),("exclude",PathType::EXCLUDE)]);
        let d = new.get(&*self.args[0]);
        let file_manager = file_manager.add(self.args[1].as_str(), d.unwrap());
    }

    fn remove_dir (self) {
        let file_manager = FileManager::default().load();
        let new = HashMap::from([("include",PathType::INCLUDE),("exclude",PathType::EXCLUDE)]);
        let d = new.get(&*self.args[0]);
        let file_manager = file_manager.add(self.args[1].as_str(), d.unwrap());
    }

    fn clean (self) {
        let file_manager = FileManager::default().load();
        let cleaner = FileCleaner{file_manager: file_manager,max_file_age: 40};
        cleaner.clean();
    }

}


fn parse_args() -> Result<CliAction,String> {
    let action = args().nth(1).expect("No valid action");
    match action.as_str() {
        "add_dir" => {
            let path_type = args().nth(2).expect("no path given");
            let path = args().nth(3).expect("no pattern given");
            if !["include", "exclude"].contains(&path_type.as_str()){
                return Err("This is shit".to_string())
            }
            let s = CliAction{action:Actions::ADD_DIR,args:vec![path_type,path]};      
            Ok(s)
        },
        "remove_dir" => {
            let path_type = args().nth(2).expect("no path given");
            let path = args().nth(3).expect("no pattern given");
            if !["include", "exclude"].contains(&path_type.as_str()){
                return Err("This is shit".to_string())
            }
            let s = CliAction{action:Actions::REMOVE_DIR,args:vec![path_type,path]};   
            Ok(s)
        },
        "clean" => {
            let s = CliAction{action:Actions::CLEAN,args:vec![]};
            Ok(s)
        }
        _ => {
            return Err("Not a valid action".to_string());
        }
    }

}

fn main() {
    let db = parse_args();
    // println!("{:?}", &db.ok().unwrap());
    let action = db.unwrap();
    action.run_action();
    // let s ="/Users/arvidbushati/Desktop/Projects/Jarvis";
    // let s = "/Users/arvid/PycharmProjects/Jarvis";
    // let typed = PathType::INCLUDE;
    
    // let file_manager = FileManager::default().load();
    // let file_manager = file_manager.add(&path, action_type);

    // let cleaner = FileCleaner {file_manager: file_manager,max_file_age: 40};
    // // file_manager.save();

    // cleaner.clean();
    // let current_dir = env::current_dir();

    // println!(
    //     "Entries modified in the last 24 hours in {:?}:",
    //     current_dir
    // );

    // // let mut file_manager = FileManager::default();
    // let mut file_manager = FileManager::default().load();
    // // file_manager = file_manager.load();
    // println!("{:?}", file_manager);
    // let dire =  current_dir.unwrap();
    // let files = walk_directory(dire);
    // for file in files {
    //     // println!("{:?}", file.file_name);
    //     file_manager.excluded_files.push(file);
        
    //     // break;
    // }
    // // file_manager.excluded_files = <Option excluded_files>;
    // // println!("{:?}", file_manager);
    
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