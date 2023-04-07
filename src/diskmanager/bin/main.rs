use std::fs;
use std::io::Read;
use std::ptr::copy_nonoverlapping;
use std::{fs::OpenOptions, io::Write};
extern crate redis;
use chrono::DateTime;
use redis::RedisError;
use redis::Commands;
use std::{thread, time::Duration};
use serde::{Serialize, Deserialize};
use thread::JoinHandle;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::prelude::*;
extern crate sqlite;
use std::io::prelude::*;
use jarvis::diskmanager::MetaData;
use std::env;

enum ManagerActions {
    WRITE_FILE,
    READ_FILE,
    UPDATE_FILE,
    DELETE_FILE
}

impl ManagerActions {
    fn get_action(s: &str) -> fn(&mut DiskManager, ManagerActionsEntry)  {
        match s.to_uppercase().as_str() {
            "WRITE_FILE"=> return DiskManager::write_file,
            "READ_FILE" => return DiskManager::read_file,
            _ => return DiskManager::write_file
        }

    }
}
#[derive(Serialize, Deserialize)]
struct ManagerActionsEntry {
    actionType: String,
    fileKey: Option<String>, 
    fileData: Option<File>
}

#[derive(Serialize, Deserialize)]
struct File {
    fileName: String,
    saved_md5:String,
    request: Vec<u8>,
}
#[derive(PartialEq,Clone)]
pub enum ManagerStates {
    WORKING,
    FREE
}
// #[derive(Clone)]
struct DiskManagerPool {
    managers: Vec<DiskManager>,
    threads: HashMap<u8,JoinHandle<()>>
}
#[derive(Clone)]
pub struct DiskManager {
    pub id: u8,
    pub state: ManagerStates,
}

fn main()  {
    let mut pool = DiskManagerPool::new(10);
    let redis = env::var("redis").unwrap();
    let redis_url = format!("redis://{}:6379",redis);
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection().unwrap();
    let key = "upload_queue";

    loop {
        let data:Result<String,RedisError> = con.lpop(key,None);
        if data.is_err(){
            println!("Nothing in queue, sleeping");
            let _ = &pool.free_managers();
            thread::sleep(Duration::from_secs(4));
            continue;
        }
        let _ = &pool.free_managers();


        let data = data.unwrap();
        let entry = serde_json::from_str::<ManagerActionsEntry>(&data).unwrap();
        // let file_data = entry.fileData;

        let running = &pool.perform_action(entry.actionType.to_string(),entry);

        if running.to_owned() == false {
            println!("Reenqueueing");
            let _:redis::RedisResult<()> = con.lpush(key.to_string(),data.clone());
        }
    }
}

impl DiskManagerPool {
    fn new(max_number_managers:u8) -> Self {
        let mut manangers = vec![];
        for i in 1..=max_number_managers{
            manangers.push(DiskManager::new(i))
        }
        DiskManagerPool { managers: manangers, threads: HashMap::new() }
    }


    fn free_managers(&mut self){
        let mut to_delete = vec![];
        for (id, thread) in &mut self.threads {
            if thread.is_finished(){
                for m in &mut self.managers {
                    if &m.id == id {
                        m.state = ManagerStates::FREE;
                        println!("Changing thread worker {:?} working state",id);
                    }
                }
                to_delete.push(id.clone());
            }
        }
        for i in to_delete {
            self.threads.remove(&i);
        }
    }
    
    fn perform_action(&mut self, action: String, data: ManagerActionsEntry) -> bool {
        for mut manager in &mut self.managers{
            if manager.state == ManagerStates::FREE {
                let d = data;
                let mut m = manager.clone();
                let action_function = ManagerActions::get_action(&action);
                
                manager.state = ManagerStates::WORKING;
                let t = thread::spawn(move || {
                    action_function(&mut m, d);
                });
                self.threads.insert(manager.id, t);
                return true
            }
        }
        println!("No free workers");
        return false;
    }    
}

// #[derive(Debug,Serialize, Deserialize)]
// struct MetaData {
//     file_id: String,
//     file_key: String,
//     insert_time: String,
//     // file_type: String
// }


impl DiskManager {
    fn new (id: u8) -> Self {
        DiskManager { id: id, state: ManagerStates::FREE}
    }

    fn create_metadata(&self, data: &File) -> String {

        //Check if key == file_path exist
        let file_id = Uuid::new_v4();
        let string_file_id = file_id.to_string();
        let file_key = &data.fileName;
        let insert_time = Utc::now().to_string();
        let md = MetaData{file_id:string_file_id.clone(),file_key:file_key.clone(),insert_time:insert_time};
        md.save();

        string_file_id
    }

    fn write_file(&mut self, data: ManagerActionsEntry) {
        if data.fileData.is_none() {
            return 
        }
        let d = data.fileData.unwrap();
        println!("Working from {:?}",&self.id);
        let file_id = self.create_metadata(&d);
        
        println!("File Name {:?}",d.fileName);
        let file_bytes = d.request;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_id).unwrap();
        let _ = file.write_all(&file_bytes).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    fn read_file(&mut self, data: ManagerActionsEntry) {
        if data.fileKey.is_none() {
            return
        }
        let key = data.fileKey.unwrap();
        let s = format!("Select * from metadata where id = '{}' ",key);
        let connection = sqlite::open("jarvis.db").unwrap();
        // let stmt = connection.prepare(s).unwrap();
        for row in connection
            .prepare(s.clone())
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap()){
                let e: &str = row.read("json_data");
                let h:MetaData = serde_json::from_str(e).unwrap();
                println!("{:?}",&h);
                let fil_id = h.file_id;
                let mut file = OpenOptions::new()
                .read(true)
                .open(fil_id).unwrap();
                
                let mut buf = vec![];
                file.read_to_end(&mut buf);
                // return buf

            
        }
        
    }

    pub fn read_file_web(&mut self, fileKey: String) -> Option<Vec<u8>> {
    
        let key = fileKey;
        let s = format!("Select * from metadata where id = '{}' ",key);
        let connection = sqlite::open("jarvis.db").unwrap();
        // let stmt = connection.prepare(s).unwrap();
        for row in connection
            .prepare(s.clone())
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap()){
                let e: &str = row.read("json_data");
                let h:MetaData = serde_json::from_str(e).unwrap();
                println!("{:?}",&h);
                let fil_id = h.file_id;
                let mut file = OpenOptions::new()
                .read(true)
                .open(fil_id).unwrap();
                
                let mut buf = vec![];
                file.read_to_end(&mut buf);
                return Some(buf)

            
        }
        return None
        
    }

}
