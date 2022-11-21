mod filesystem;
use filesystem::{Directory,File};

struct FileCleaner {
    file_manager: FileManager,
    max_file_age: u64,
    // to_delete_queue: PathBuf
}

impl FileCleaner{
    fn clean(&self){
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
        }

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