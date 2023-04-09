use bytes::{Bytes, Buf};
use std::io::Read;
use std::process::Command;
use std::{fs::OpenOptions, io::Write};
use serde::{Serialize, Deserialize};
use actix_web::{get, post, web, App ,HttpRequest, HttpResponse, HttpServer, Responder, http};
extern crate redis;
use redis::Commands;
use std::collections::HashMap;
use uuid::Uuid;
use md5;
use std::str;
use jarvis::diskmanager::MetaData;
use http::StatusCode;
use std::env;
use jarvis::syner::FileUploadData;
use jarvis::diskmanager::ManagerActionsEntry;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn get_upload_file_data(id: &str) -> FileUploadData {
    let redis = env::var("redis").unwrap();
    let redis_url = format!("redis://{}:6379",redis);
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection().unwrap();
    let key = format!("upload_{}",id);
    println!("{:?}", key);
    let data :Result<String, redis::RedisError> = con.get(&key);

    let uploaded_file = serde_json::from_str::<FileUploadData>(&data.unwrap());
    uploaded_file.unwrap()
}

fn set_upload_file(key: String, value: String) -> redis::RedisResult<()> {
    let redis = env::var("redis").unwrap();
    println!("redis {}",redis);
    let redis_url = format!("redis://{}:6379",redis);
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection()?;
    let _ : () = con.set(&key,value)?;
    let _ : () = con.expire(&key,12)?;
    Ok(())
}

fn push_upload(data: FileUploadData,file_bytes: Vec<u8>) -> String {
    let redis = env::var("redis").unwrap();
    let redis_url = format!("redis://{}:6379",redis);
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection().unwrap();
    let upload_entry = ManagerActionsEntry {
                                                    actionType: "write_file".to_string(),
                                                    fileData: Some(data),
                                                    file_bytes:file_bytes
                                                };

    let d = serde_json::to_string(&upload_entry).unwrap();
    let _:redis::RedisResult<()> = con.lpush("upload_queue".to_string(),d);
    println!("Pushed to redis for diskmanager");
    "ok".to_string()
}

#[post("/upload_file_data/{id}")]
async fn upload_file_data(file_bytes: web::Bytes,tid: web::Path<(String,)>) -> impl Responder {
    let uploaded_file = get_upload_file_data(&tid.0);
    let saved_md5 = uploaded_file.file_md5.clone();
    
    let digest = format!("{:x}",md5::compute(&file_bytes));
    if &saved_md5 != &digest{
        return HttpResponse::BadRequest().body("Body isnt equal to file metadata md5")
    }
    push_upload(uploaded_file,file_bytes.to_vec());
    HttpResponse::Ok().body("File Uploaded")
}



#[post("/upload_file")]
async fn upload_file(request: web::Json<FileUploadData>) -> impl Responder {
    let file_upload_data: &FileUploadData = &request;
    let str_data = serde_json::to_string(file_upload_data).unwrap();

    let upload_id = Uuid::new_v4().to_string();
    let upload_key = format!("upload_{}",&upload_id);


    set_upload_file(upload_key, str_data);
    HttpResponse::Ok().body(upload_id)
}

#[get("/read_file/{file_key}")]
async fn read_file(request: web::Path<(String,)> ) -> HttpResponse {
    let key = request.clone().0;
    let s = format!("Select * from metadata where public_file_path = '{}' ",key);
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
            let file_path = h._internal_file_path;
            let mut file = OpenOptions::new()
            .read(true)
            .open(file_path).unwrap();
            
            let mut buf = vec![];
            file.read_to_end(&mut buf);
            
            return HttpResponse::build(StatusCode::OK)
            .content_type("application/octet-stream")
            .header("Content-Disposition", format!("attachment; filename=\"{}\"",h.file_name))
            .body(buf)
        
    }
    HttpResponse::Ok().body("hi".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // server();
    // tcpconnect();
    let output = Command::new("hostname")
    .output()
    .expect("failed to execute process");
    println!("{:?}",String::from_utf8_lossy(&output.stdout));
    
    if String::from_utf8_lossy(&output.stdout) == "Arvids-MacBook-Pro.local\n" {
        println!("here");
        std::env::set_var("redis", "localhost");
        println!("{:?}",std::env::var("redis").unwrap());
    }

    HttpServer::new(|| {
        App::new()
            .app_data(web::PayloadConfig::new(1 << 25))
            .service(hello)
            .service(upload_file)
            .service(upload_file_data)
            .service(read_file)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

