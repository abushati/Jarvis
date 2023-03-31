use std::{fs::OpenOptions, io::Write};
extern crate redis;
use redis::RedisError;
use redis::Commands;
use std::{thread, time::Duration};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct File {
    fileName: String,
    saved_md5:String,
    request: Vec<u8>,
}
#[derive(PartialEq)]
enum ManagerStates {
    WORKING,
    FREE
}
struct DiskManagerPool {
    managers: Vec<DiskManager>,
    max_number_managers: u8,
}
struct DiskManager {
    id: u8,
    state: ManagerStates,
}

fn main()  {
    let pool = DiskManagerPool::new(3);


    loop {
        let client = redis::Client::open("redis://localhost:6379").unwrap();
        let mut con = client.get_connection().unwrap();
        let key = "upload_queue";
        let data:Result<String,RedisError> = con.lpop(key,None);
        
        if data.is_err(){
            println!("Nothing in queue, sleeping");
            thread::sleep(Duration::from_secs(4));
            continue;
        }
        &mut pool.write_file(data.unwrap());
    }
}

impl DiskManagerPool {
    fn new(max_number_managers:u8) -> Self {
        let mut manangers = vec![];
        for i in 1..max_number_managers{
            manangers.push(DiskManager::new(i))
        }
        DiskManagerPool { managers: manangers, max_number_managers: max_number_managers }
    }

    fn write_file(&mut self, data: String) {
        for mut i in &self.managers{
            if i.state == ManagerStates::FREE {
                // i.state = ManagerStates::WORKING.
                

            }
        }
    }
    
}

impl DiskManager {
    fn new (id: u8) -> Self {
        DiskManager { id: id, state: ManagerStates::FREE}
    }

    fn write_file(self, data: String) {
        let data = data;
        println!("{:?}",data);
        let d = serde_json::from_str::<File>(&data).unwrap();
        let file_bytes = d.request;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(d.fileName).unwrap();
        let _ = file.write_all(&file_bytes).unwrap();
        thread::sleep(Duration::from_secs(4));
    }
}
