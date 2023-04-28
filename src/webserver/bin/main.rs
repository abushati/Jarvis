use actix_web::{get, post, delete, web, App ,HttpRequest, HttpResponse, HttpServer, Responder, http};
use http::StatusCode;
use std::io::Read;
use std::process::Command;
use std::fs::OpenOptions;
use std::str;
use std::env;
extern crate redis;
use redis::Commands;
use uuid::Uuid;
use md5;
use serde::{Serialize, Deserialize};


use jarvis::diskmanager::MetaData;
use jarvis::syner::FileUploadData;
use jarvis::diskmanager::ManagerActionsEntry;
use jarvis::diskmanager::ManagerAction;
use jarvis::diskmanager::


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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

fn set_upload_file_pending(key: String, value: String) -> redis::RedisResult<()> {
    let redis = env::var("redis").unwrap();
    println!("redis {}",redis);
    let redis_url = format!("redis://{}:6379",redis);
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection()?;
    let _ : () = con.set(&key,value)?;
    let _ : () = con.expire(&key,12)?;
    Ok(())
}

fn queue_diskmanager_acton(data: ManagerActionsEntry) -> String {
    let redis = env::var("redis").unwrap();
    let redis_url = format!("redis://{}:6379",redis);
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection().unwrap();

    let d = serde_json::to_string(&data).unwrap();
    let _:redis::RedisResult<()> = con.lpush("upload_queue".to_string(),d);
    println!("Pushed to action {:?} to redis queue for diskmanager", &data.action_type);
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
    let action_type = ManagerAction::WriteFile;
    let data = WriteFile { file_name: uploaded_file.file_name,
        file_path: uploaded_file.file_path,
        file_md5: uploaded_file.file_md5,
        file_bytes: file_bytes.to_vec(),
        user: "1".to_string(),
        basket: "1".to_string() };
  
    let upload_entry = ManagerActionsEntry { action_type: action_type, data: data };

    queue_diskmanager_acton(upload_entry);
    HttpResponse::Ok().body("File Uploaded")
}


#[post("/upload_file")]
async fn upload_file(request: web::Json<FileUploadData>) -> impl Responder {
    let file_upload_data: &FileUploadData = &request;
    let str_data = serde_json::to_string(file_upload_data).unwrap();

    let upload_id = Uuid::new_v4().to_string();
    let upload_key = format!("upload_{}",&upload_id);

    set_upload_file_pending(upload_key, str_data);
    HttpResponse::Ok().body(upload_id)
}

#[get("/read_file/{file_key}")]
async fn read_file(re: HttpRequest, request: web::Path<(String,)> ) -> HttpResponse {
    let key = request.clone().0;
    println!("Path {:?}",key);
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
    HttpResponse::NotFound().body("hi".to_string())
}


#[delete("/delete_file/{file_key}")]
async fn delete_file(re: HttpRequest, request: web::Path<(String,)> ) -> HttpResponse {

    let file_key = request.clone().0;
    let action_type = ManagerAction::WriteFile;
    let data = DeleteFile{ file_pub_key: file_key };

    let delete_entry = ManagerActionsEntry { action_type: action_type, data: data };
    
    println!("queuing delete here");
    queue_diskmanager_acton(delete_entry);
    HttpResponse::Ok().body("File queued to be deleted")
}


#[derive(Serialize, Deserialize)]
enum BasketPermissions {
    VIEW,
    EDIT,
    CREATE,
    DELETE
}

//Todo: should the owner be a string or an float
#[derive(Serialize, Deserialize)]
struct Basket {
    name: String,
    owner: String,
    permissions: Vec<BasketPermissions>,
    create_at: String
}

#[post("/basket")]
async fn add_basket(basket_data: web::Json<Basket>) -> HttpResponse {
    let basket_data = basket_data.0;

    HttpResponse::Ok().body("File Uploaded")    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .service(delete_file)
            .service(add_basket)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

