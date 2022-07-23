use std::{env, fs};


#[derive(Debug)]
struct File {
    path: std::path::PathBuf,
    file_name: std::ffi::OsString,
    last_accessed: std::time::SystemTime,
    last_modified:std::time::SystemTime,
    created: std::time::SystemTime,
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
    }
    
}


fn walk_directory(directory:std::path::PathBuf) -> Vec<File> {
    let mut current_dir_files: Vec<File> = Vec::new();
    let directs = fs::read_dir(directory).unwrap();
    for  entry in directs {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
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