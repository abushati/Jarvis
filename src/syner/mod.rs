use reqwest::blocking::Client;
use std::{collections::HashMap};
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use serde::{Deserialize,Serialize};
// extern crate url;
use url::form_urlencoded::byte_serialize;

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

pub struct syncer {
    pub api_client: reqwest::blocking::Client,
    pub server_url: String,
}

impl syncer {
    pub fn new() -> Self{
        let server = "http://127.0.0.1:8080".to_string();
        let client = Client::new();
        syncer { api_client:client, server_url: server }
    }

    fn url_endpoint(&self, endpoint: Vec<String>) -> String {
        let mut v:Vec<String> = vec![];
        for i in endpoint {
            let encoded =byte_serialize(i.as_bytes()).collect::<String>();
            v.push(encoded);
        }
        let endpoint = v.join("/");
        return format!("{}/{}",self.server_url,endpoint);
    }

    fn send_file_bytes(&self, file_id:String, bytes:&Vec<u8>) {
        let endpoint = vec!["upload_file_data".to_string(),file_id];
        let post = self.api_client.post(&self.url_endpoint(endpoint))
        .body(bytes.to_owned())
        .send().unwrap();
        println!("Status: {}", post.status());
    }

    fn send_file_meta(&self, file_data:FileUploadData) -> String {
        // let json_value: Value = serde_json::to_value(file_data)?;
        let file_data = file_data.to_hashmap();
        let endpoint = vec!["upload_file".to_string()];
        let post = self.api_client
        .post(self.url_endpoint(endpoint))
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

    pub fn upload_file(&self, file_path: &String){
        let path = Path::new(file_path);
        if !path.is_file(){
            println!("Path provided isn't a file");
            return;
        }
        self._upload_file(path);
    }

    pub fn upload_directory(&self, directory: &String) {
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

    pub fn read_file(&self, file_key: &String) {
        let endpoint = vec!["read_file".to_string(),file_key.to_owned()];
        let res = self.api_client.get(self.url_endpoint(endpoint))
        .send()
        .unwrap();
        
        let headers = res.headers().clone();
        let content_disposition = headers.get("content-disposition").unwrap();
        
        let h: Vec<&str> = content_disposition.to_str().unwrap().split(";").collect();
        
        let mut file_name = h.get(1).unwrap().to_string();
        let h:Vec<&str> = file_name
        .split("=")
        .collect();
        
        let name = h.get(1).unwrap().to_string();
        let name = name
        .strip_prefix("\"")
        .unwrap();
        let name = name
        .strip_suffix("\"")
        .unwrap();
        println!("{}",name);
        

        let file_bytes = res.bytes().unwrap().to_vec();
        
        //Todo: add dowload dir config
        let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("/Users/arvidbushati/Downloads/{}",name)).unwrap();

        let _ = file.write_all(&file_bytes);

        
    }

    pub fn delete_file (self, file_key: &String) {
        let endpoint = vec!["delete_file".to_string(),file_key.to_owned()];
        let res = self.api_client.delete(self.url_endpoint(endpoint))
        .send()
        .unwrap();
        println!("{:?}",res.text());
    }

}

