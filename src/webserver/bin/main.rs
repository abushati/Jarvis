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

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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

fn get_upload_file_data(id: &str) -> HashMap<String,String> {
    let redis = env::var("redis").unwrap();
    let redis_url = format!("redis://{}:6379",redis);
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection().unwrap();
    let key = format!("upload_{}",id);
    println!("{:?}", key);
    let data :Result<HashMap<String,String>, redis::RedisError> = con.hgetall(key);
    let uploaded_file = data.unwrap();

    
    return uploaded_file
}
#[derive(Debug,Deserialize)]
struct FileUpload {
    fileName: String,
    md5: String,
    // mimeType: String
}

fn set_upload_file(key: String, value: Vec<(&str,&String)>) -> redis::RedisResult<()> {
    let redis = env::var("redis").unwrap();
    println!("redis {}",redis);
    let redis_url = format!("redis://{}:6379",redis);
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection()?;
    let _ : () = con.hset_multiple(&key,&value)?;
    let _ : () = con.expire(&key,12)?;
    Ok(())
}

fn push_upload(data:  File) -> String {
    let redis = env::var("redis").unwrap();
    let redis_url = format!("redis://{}:6379",redis);
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection().unwrap();
    let upload_entry = ManagerActionsEntry {
                                                    actionType: "write_file".to_string(),
                                                    fileData: Some(data),
                                                    fileKey:None
                                                };

    let d = serde_json::to_string(&upload_entry).unwrap();
    let _:redis::RedisResult<()> = con.lpush("upload_queue".to_string(),d);
    println!("Pushed to redis for diskmanager");
    "ok".to_string()
}

#[post("/upload_file_data/{id}")]
async fn upload_file_data(request: web::Bytes,tid: web::Path<(String,)>) -> impl Responder {
    let  uploaded_file = get_upload_file_data(&tid.0);
    let saved_md5 = uploaded_file.get("md5").unwrap().to_string();
    let fileName = uploaded_file.get("fileName").unwrap().to_string();
    //GenericArray's docs are here 57 and it implements std::fmt::UpperHex and std::fmt::LowerHex
    let digest = format!("{:x}",md5::compute(&request));
    println!("{:?}",&saved_md5);
    println!("{:?}",&digest);
    if &saved_md5 != &digest{
        return HttpResponse::BadRequest().body("Body isnt equal to file metadata md5")
    }
    println!("good md5");
    // let byte_string = format!("{:?}",request.to_vec());
    // println!("{:?}", byte_string);
    let data = File{fileName:fileName,saved_md5:saved_md5,request:request.to_vec()};
    // let data:HashMap<&str, _> = HashMap::from([("fileName",fileName), ("saved_md5",saved_md5),("request", )]);
    push_upload(data);
    HttpResponse::Ok().body("File Uploaded")
}



#[post("/upload_file")]
async fn upload_file(request: web::Json<FileUpload>) -> impl Responder {
    let e = vec![("fileName",&request.fileName),("md5",&request.md5)];
    let id = Uuid::new_v4();
    let s_id = id.to_string();
    let upload_key = format!("upload_{}",&s_id);

    set_upload_file(upload_key, e);
    HttpResponse::Ok().body(s_id)
}

#[get("/read_file/{file_key}")]
async fn read_file(request: web::Path<(String,)> ) -> HttpResponse {
    // let client = redis::Client::open("redis://localhost:6379").unwrap();
    // let mut con = client.get_connection().unwrap();
    // let upload_entry = ManagerActionsEntry {
    //                                                 actionType: "read_file".to_string(),
    //                                                 fileData: None,
    //                                                 fileKey: Some(request.clone().0)
    //                                             };

    // let d = serde_json::to_string(&upload_entry).unwrap();
    // let _:redis::RedisResult<()> = con.lpush("upload_queue".to_string(),d);
    // println!("Pushed to redis for diskmanager");

    let key = request.clone().0;
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
            
            return HttpResponse::build(StatusCode::OK)
            .content_type("application/octet-stream")
            .header("Content-Disposition", format!("attachment; filename=\"{}\"",h.file_key))
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
            .service(echo)
            .service(upload_file)
            .service(upload_file_data)
            .service(read_file)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

