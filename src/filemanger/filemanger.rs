mod filemanger;
#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    path: std::path::PathBuf,
    file_name: String,
    last_accessed: std::time::SystemTime,
    last_modified:std::time::SystemTime,
    created: std::time::SystemTime,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Directory {
    path: std::path::PathBuf,
    files: Vec<File>,
    child_directories: Vec<Directory>,
}

#[derive(Debug,Serialize, Deserialize, Default)]
pub struct FileManager {
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
    //Need a way to detect duplicates and stop
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
                } else {
                    println!("input is not a file")
                }
            }
        }
        self.save();
        // println!("{:?}", typed);
        self
    }

    // fn remove(mut self, section: &file_manager_section, path: &str ) -> Self
    // {
    //     return self
    // }
    fn remove(mut self, section: &file_manager_section, path: &str ) -> Self{
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


#[derive(Debug)]
enum file_manager_section {
    EXCLUDE_DIR,
    INCLUDE_DIR,
    EXCLUDE_FILE,
}

#[derive(Debug,PartialEq)]
enum manager_actions {
    ADD,
    REMOVE,
    RESET
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
