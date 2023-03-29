use reqwest::blocking::Client;
use std::path::Path;
use std::fs::{read_dir,File,write, DirEntry};
use std::io::prelude::*;

pub struct syncer {
    pub destination: String,
}

impl syncer {
    fn send_file_bytes(&self, bytes:&Vec<u8>) {
        let client = Client::new();
        let post = client.post("http://127.0.0.1:8080/upload_file_data/adfasf")
        .body(bytes.to_owned())
        .send().unwrap();
        println!("Status: {}", post.status());
    }

    pub fn upload_file(&self, file: DirEntry ) {

        println!("In default function");
        let file_name = file.file_name();
        let file_path = file.path();
        let file_meta = file.metadata().unwrap();

        let mut output_path = format!("/Users/arvidbushati/Desktop/Projects/Jarvis/{}",file_name.to_str().unwrap());
        let mut file = File::open(&file_path).unwrap();
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(& mut buf);

        self.send_file_bytes(&buf);
        
        return
    }

    pub fn sync_directory(&self, directory: &String) {
        let path = Path::new(&directory);
        let i = read_dir(path);
        match i {
            Ok(a) => {
                for s in a {
                    let entry = s.unwrap();

                    // write(&output_path, &buf);
                    
                    
                    println!("Successfully created file at {:?}", &output_path);
                    // let value = json!({
                    //     "code": 200,
                    //     "success": true,
                    //     "payload": {
                    //         "features": [
                    //             "serde",
                    //             "json"
                    //         ]
                    //     }
                    // });
                    // println!("{:?}", value.to_string());
                    
                }
            }
            Err(e) => {println!("what is u doing? {:?}",e)}
        }

    }

}
