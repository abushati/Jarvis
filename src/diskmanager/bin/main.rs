use std::{fs::OpenOptions, io::Write};
extern crate redis;
use redis::RedisError;
use redis::Commands;
use std::{thread, time::Duration};
use serde::{Serialize, Deserialize};
use thread::JoinHandle;
use std::collections::HashMap;
// use std::sync::mpsc;

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
    max_number_managers: u8,
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
            &pool.clear_threads();
            thread::sleep(Duration::from_secs(4));
            continue;
        }

        let data = data.unwrap();
        &pool.clear_threads();
        let running = &pool.write_file(&data.clone());

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
        DiskManagerPool { managers: manangers, max_number_managers: max_number_managers, threads: HashMap::new() }
    }

    fn clear_threads (&mut self) {
        let to_clear = &self.clear();
        &self.delete(to_clear.to_owned());
    }
    fn clear(&mut self) -> Vec<u8> {
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
        to_delete
    }
    
    fn delete (&mut self, vec: Vec<u8>){
        for i in vec {
            self.threads.remove(&i);
        }
    }

    fn write_file(&mut self, data: &String) -> bool {

        for mut manager in &mut self.managers{
            if manager.state == ManagerStates::FREE {
                let d = data.clone();

                let mut m = manager.clone();
                manager.state = ManagerStates::WORKING;
                let t = thread::spawn(move || {
                    m.write_file(d);

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

    fn write_file(&mut self, data: String) {
        println!("Working from {:?}",&self.id);
        let data = data;
        
        // println!("{:?}",data);
        let d = serde_json::from_str::<File>(&data).unwrap();
        println!("File Name {:?}",d.fileName);
        let file_bytes = d.request;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(d.fileName).unwrap();
        let _ = file.write_all(&file_bytes).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
