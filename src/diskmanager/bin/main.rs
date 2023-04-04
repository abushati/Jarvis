use std::fs;
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
// use std::sync::mpsc;
use std::str::FromStr;
enum ManagerActions {
    WRITE_FILE,
    READ_FILE,
    UPDATE_FILE,
    DELETE_FILE
}

impl ManagerActions {
    fn get_action(s: &str) -> fn(&mut DiskManager, File)  {
        match s.to_uppercase().as_str() {
            "WRITE_FILE"=> return DiskManager::write_file,
            _ => return DiskManager::write_file
        }

    }
}
#[derive(Serialize, Deserialize)]
struct ManagerActionsEntry {
    actionType: String,
    fileData: File
}

#[derive(Serialize, Deserialize)]
struct File {
    fileName: String,
    saved_md5:String,
    request: Vec<u8>,
}
#[derive(PartialEq,Clone)]
enum ManagerStates {
    WORKING,
    FREE
}
// #[derive(Clone)]
struct DiskManagerPool {
    managers: Vec<DiskManager>,
    threads: HashMap<u8,JoinHandle<()>>
}
#[derive(Clone)]
struct DiskManager {
    id: u8,
    state: ManagerStates,
}

fn main()  {
    let mut pool = DiskManagerPool::new(10);
    let client = redis::Client::open("redis://localhost:6379").unwrap();
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
        let file_data = entry.fileData;

        let running = &pool.perform_action(entry.actionType.to_string(),file_data);

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
    
    fn perform_action(&mut self, action: String, data: File) -> bool {
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

#[derive(Debug)]
struct MetaData {
    file_id: String,
    file_key: String,
    insert_time: String,
    // file_type: String
}

impl MetaData {
    fn save(self) {
        println!("{:?}",self)
    }
}
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

    fn write_file(&mut self, data: File) {

        println!("Working from {:?}",&self.id);
        let d = data;
        let file_id = self.create_metadata(&d);
        
        // println!("{:?}",data);
        // let d = serde_json::from_str::<File>(&data).unwrap();
        println!("File Name {:?}",d.fileName);
        let file_bytes = d.request;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_id).unwrap();
        let _ = file.write_all(&file_bytes).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
