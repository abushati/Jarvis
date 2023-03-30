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
fn main()  {
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
        let data = data.unwrap();
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
