use std::io::Read;

use std::{fs::OpenOptions, io::Write};
extern crate redis;
use jarvis::syner::FileUploadData;
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
use jarvis::diskmanager::{MetaData, ManagerActionsEntry};
use std::env;
use std::process::Command;
use jarvis::diskmanager::ManagerAction;

static file_directory: &str = "/private/tmp/file_directory";


fn diskmanager_action_function(s: &ManagerAction) -> fn(&mut DiskManager, ManagerActionsEntry)  {
        match s {
            ManagerAction::WriteFile => return DiskManager::write_file,
            ManagerAction::ReadFile => return DiskManager::read_file,
            ManagerAction::DeleteFile => return DiskManager::delete_file,
            _ => return DiskManager::read_file
        }
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
    let output = Command::new("hostname")
    .output()
    .expect("failed to execute process");
    println!("{:?}",String::from_utf8_lossy(&output.stdout));
    
    if String::from_utf8_lossy(&output.stdout) == "Arvids-MacBook-Pro.local\n" {
        println!("here");
        std::env::set_var("redis", "localhost");
        println!("{:?}",std::env::var("redis").unwrap());
    }
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

        let running = &pool.perform_action(entry);

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
        //Ensure that file directory exist
        let exist = std::path::Path::exists(std::path::Path::new(file_directory));
        if !exist {
            std::fs::create_dir(file_directory).unwrap();
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
    
    fn perform_action(&mut self, actions_entry: ManagerActionsEntry) -> bool {
        for mut manager in &mut self.managers{
            if manager.state == ManagerStates::FREE {
                let action_type = &actions_entry.action_type;
                let action_function = diskmanager_action_function(action_type);

                let mut m = manager.clone();
                let data = actions_entry;
                
                manager.state = ManagerStates::WORKING;
                let t = thread::spawn(move || {
                    action_function(&mut m, data);
                });
                self.threads.insert(manager.id, t);
                return true
            }
        }
        println!("No free workers");
        return false;
    }    
}

impl DiskManager {
    fn new (id: u8) -> Self {
        DiskManager { id: id, state: ManagerStates::FREE}
    }

    fn create_metadata(&self, data: &FileUploadData) -> MetaData {
        
        //Check if key == file_path exist
        let _internal_file_id = Uuid::new_v4();
        let string_internal_file_id = _internal_file_id.to_string();
        
        let insert_time = Utc::now().to_string();

        let entry_path = format!("{}/{}",file_directory, _internal_file_id);
        let mut md = MetaData{
            file_name: data.file_name.clone(),
            public_file_path: data.file_path.clone(),
            _internal_file_id: string_internal_file_id,
            _internal_file_path: entry_path,
            insert_time: insert_time.clone(),
            update_time: insert_time.clone(),
            user: None
        };
        let _ = &md.save();
        return md
    }

    fn delete_file(&mut self, data: ManagerActionsEntry) {
        if data.file_pub_key.is_none() {
            println!("Can't delete a file with no file key provided");
        }
        let file_key = data.file_pub_key.unwrap();
        let meta_data = MetaData::get_key_meta(file_key).unwrap();
        std::fs::remove_file(&meta_data._internal_file_path).unwrap();
        meta_data.delete();

    }

    fn write_file(&mut self, data: ManagerActionsEntry) {
        if data.fileData.is_none() || data.file_bytes.is_none() {
            println!("Can't save file bc fileData or file bytes are missing");
            return 
        }
        
        let meta_data = self.create_metadata(&data.fileData.unwrap());
        println!("Created meta data for file {:?} on worker {:?}", meta_data.public_file_path,&self.id);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(meta_data._internal_file_path).unwrap();
        let _ = file.write_all(&data.file_bytes.unwrap()).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    fn read_file(&mut self, data: ManagerActionsEntry) {
        
    }

    pub fn read_file_web(&mut self, public_file_key: String) -> Option<Vec<u8>> {
        let s = format!("Select * from metadata where public_file_path = '{}' ",public_file_key);
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
                let in_file_path = h._internal_file_path;
                let mut file = OpenOptions::new()
                .read(true)
                .open(in_file_path).unwrap();
                
                let mut buf = vec![];
                file.read_to_end(&mut buf);
                return Some(buf)
        }
        return None
        
    }

}
