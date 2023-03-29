use reqwest::blocking::Client;
use std::collections::HashMap;
use std::path::Path;
use std::fs::{read_dir,File,write, DirEntry};
use std::io::prelude::*;
use serde_json::json;
pub struct syncer {
    pub destination: String,
}

impl syncer {
    fn send_file_bytes(&self, file_id:String, bytes:&Vec<u8>) {
        let client = Client::new();
        let post = client.post(format!("http://127.0.0.1:8080/upload_file_data/{}",file_id))
        .body(bytes.to_owned())
        .send().unwrap();
        println!("Status: {}", post.status());
    }

    fn send_file_meta(&self, file_data:HashMap<&str,&str>) -> String {
        // let json_value: Value = serde_json::to_value(file_data)?;

        let client = Client::new();
        let post = client
        .post("http://127.0.0.1:8080/upload_file")
        .json(&file_data)
        .send().unwrap();
        // println!("{:?}",&post.text());
        post.text().unwrap()
    }

    pub fn upload_file(&self, file: DirEntry ) {

        println!("In default function");
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        let file_path = file.path();
        
        // let mut output_path = format!("/Users/arvidbushati/Desktop/Projects/Jarvis/{}",file_name.to_str().unwrap());
        let mut file = File::open(&file_path).unwrap();
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(& mut buf);

        let file_md5 = format!("{:x}",md5::compute(&buf));
        let file_md5 = file_md5.as_str();
        let file_key = file_path.clone().to_str().unwrap();

        let file_meta_data = HashMap::from([
            ("fileName",file_name),
            ("md5",file_md5),
            ]);

        let file_id = self.send_file_meta(file_meta_data);
        self.send_file_bytes(file_id, &buf);
        
        return
    }

    pub fn sync_directory(&self, directory: &String) {
        let path = Path::new(&directory);
        let i = read_dir(path);
        match i {
            Ok(a) => {
                for s in a {
                    let entry = s.unwrap();
                    if entry.metadata().unwrap().is_file(){
                        self.upload_file(entry);
                    }
                    
                    // write(&output_path, &buf);
                                        
                    println!("Successfully synced directory {:?}",&path);

                    
                }
            }
            Err(e) => {println!("what is u doing? {:?}",e)}
        }

    }

}
