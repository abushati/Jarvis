
// mod filesystem;
use crate::filesystem::{Directory,File, FileManager};
extern crate sqlite;
use std::fs::OpenOptions;
use chrono::{DateTime, Utc, offset};
// use std::fs::File;
use std::io::prelude::*;
pub struct FileCleaner {
    pub file_manager: FileManager,
    pub max_file_age: u64,
    pub db: Option<sqlite::Connection>,
}

impl FileCleaner{
    fn db_connect (&self) -> sqlite::Connection{;
        let connection = sqlite::open("jarvis.db").unwrap();
        let query = "
        CREATE TABLE delete_queue (file_path TEXT, to_delete NUMERIC, insert_time TEXT);
        ";

        // let query = "
        // CREATE TABLE users (name TEXT, age INTEGER);
        // INSERT INTO users VALUES ('Alice', 42);
        // INSERT INTO users VALUES ('Bob', 69);
        // ";
        
        connection.execute(query);
        return connection
    }

    fn run_query(&self, query: String) -> String {
        let db = self.db.as_ref().ok_or("bad").unwrap();
        println!("{}",query);
        for row in db
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap()){
                println!("{:?}", row);
                let e: i64 = row.read("count(*)");

                println!("{}",e);

            }

            // let query = "SELECT * FROM users WHERE age > ?";

            // for row in db
            //     .prepare(query)
            //     .unwrap()
            //     .into_iter()
            //     .bind((1, 50))
            //     .unwrap()
            //     .map(|row| row.unwrap())
            // {
            //     println!("name = {}", row.read::<&str, _>("name"));
            //     println!("age = {}", row.read::<i64, _>("age"));
            // }

            return "results".to_string()
    }

    pub fn clean(mut self){
        self.db = Some(self.db_connect());
        
        println!("wrote to db");
        let dirs_to_clean = &self.file_manager.included_directories;
        for dir in dirs_to_clean{
            self.clean_dir_files(dir);
        }
    }
    fn check_excluded (&self, dir: &Directory, let_dir_to_check: &Vec<Directory> ) -> bool{
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
    fn clean_dir_files(&self, dir: &Directory ) {
        if self.check_dir_in_excluded(dir){
            println!("{:?} dir is excluded", dir.path);
            return
        }
        
        
        for file in &dir.files {
            let now: DateTime<Utc> = file.last_accessed.into();
            let file_path = file.path.to_str().unwrap();
            if self.should_delete_file(file){
                // println!("{:?}", format!("select count(*) from delete_queue where to_delete = False and file_path = '{}';",file_path));
                // let exist = self.run_query(format!("select count(*) from delete_queue where to_delete = False and file_path = '{}';",file_path));
                let exist = self.run_query(format!("select count(*) from delete_queue limit 10;"));

                // self.run_query(format!("insert into delete_queue values ({},{},{});",file_path,false,chrono::offset::Utc::now().to_string()));
                println!("{:?}", exist);
            }
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
