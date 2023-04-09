use reqwest::blocking::Client;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize,Serialize};
pub struct syncer {
    pub destination: String,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct FileUploadData {
    pub file_name: String,
    pub file_path: String,
    pub file_md5: String,
    pub user: String
}

impl FileUploadData {
    fn to_hashmap(&self) -> HashMap<&str, &str> {
        let mut map = HashMap::new();
        map.insert("file_name", self.file_name.as_str());
        map.insert("file_path", self.file_path.as_str());
        map.insert("file_md5", self.file_md5.as_str());
        map.insert("user", self.user.as_str());
        return map
    }
}


impl syncer {
    fn send_file_bytes(&self, file_id:String, bytes:&Vec<u8>) {
        let client = Client::new();
        let post = client.post(format!("http://127.0.0.1:8080/upload_file_data/{}",file_id))
        .body(bytes.to_owned())
        .send().unwrap();
        println!("Status: {}", post.status());
    }

    fn send_file_meta(&self, file_data:FileUploadData) -> String {
        // let json_value: Value = serde_json::to_value(file_data)?;
        let file_data = file_data.to_hashmap();
        let client = Client::new();
        let post = client
        .post("http://127.0.0.1:8080/upload_file")
        .json(&file_data)
        .send().unwrap();
        // println!("{:?}",&post.text());
        post.text().unwrap()
    }

    pub fn _upload_file(&self, file: &Path ) {
        let file_name = file.file_name().unwrap();
        let file_name = file_name.to_str().unwrap().to_string();

        let file_path = file.as_os_str().to_str().unwrap().to_string();
        println!("path {:?}",file_path);
        // let mut output_path = format!("/Users/arvidbushati/Desktop/Projects/Jarvis/{}",file_name.to_str().unwrap());
        let mut file = File::open(&file_path).unwrap();
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(& mut buf);

        let file_md5 = format!("{:x}",md5::compute(&buf));
        // let file_md5 = file_md5.as_str().to_string();
        let file_key = file_path.clone();
        let user = "1".to_string();

        let file_meta_data = FileUploadData{
            file_name: file_name,
            file_path: file_path,
            file_md5: file_md5,
            user: user
        };

        let file_id = self.send_file_meta(file_meta_data);
        self.send_file_bytes(file_id, &buf);
        
        return
    }

    pub fn sync_file(&self, file_path: &String){
        let path = Path::new(file_path);
        if !path.is_file(){
            println!("Path provided isn't a file");
            return;
        }
        self._upload_file(path);
    }

    pub fn sync_directory(&self, directory: &String) {
        let path = Path::new(&directory);
        let i = path.read_dir();
        match i {
            Ok(a) => {
                for s in a {
                    let entry = s.unwrap();
                    if entry.metadata().unwrap().is_file(){
                        // entry.path();
                        self._upload_file(&entry.path());
                        println!("Successfully synced directory {:?}",&path);
                    }                   
                }
            }
            Err(e) => {println!("what is u doing? {:?}",e)}
        }

    }

}
