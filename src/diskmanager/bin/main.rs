use std::{fs::OpenOptions, io::Write};
extern crate redis;
use redis::Commands;
use std::{thread, time::Duration};
#[derive(serde::Deserialize)]
struct File {
    fileName: String,
    saved_md5: String,
    request: String
}
fn main()  {
    loop {
        let client = redis::Client::open("redis://localhost:6379").unwrap();
        let mut con = client.get_connection().unwrap();
        let key = "upload_queue";
        let data= con.lpop(key,None);
        if data.is_err(){
            println!("Nothing in queue, sleeping");
            thread::sleep(Duration::from_secs(4));
            continue;
        }
        let data: String = data.unwrap();
        println!("{:?}",data);
        let d = serde_json::from_str::<File>(&data).unwrap();
        let file_bytes = d.request.as_bytes();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(d.fileName).unwrap();
        let _ = file.write_all(file_bytes).unwrap();
        thread::sleep(Duration::from_secs(4));
    }
}
