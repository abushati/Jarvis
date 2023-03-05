mod ftpss;
use ftpss::client::tcpconnect;
use ftpss::server::server;
use std::{fs::OpenOptions, io::Write};
use actix_multipart::Multipart;
use serde::Deserialize;
use actix_web::{get, post, web, App,HttpRequest, HttpResponse, HttpServer, Responder};


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


#[derive(Debug,Deserialize)]
struct FileUpload {
    fileName: String,
    // fileData: web::Bytes
}

#[post("/upload_file_data/{id}")]
async fn upload_file_data(request: web::Bytes,tid: web::Path<(u32,)>) -> impl Responder {
    println!("{:?}",tid);
    println!("{:?}",request);
    // let mut file = OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .open("foo.docx").unwrap();
    // // file.write(&request.fileData.as_bytes());
    HttpResponse::Ok().body("s")
}


#[post("/upload_file")]
async fn upload_file(request: web::Json<FileUpload>) -> impl Responder {
    println!("{:?}",request);
    let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("foo.docx").unwrap();
    // file.write(&request.fileData.as_bytes());
    HttpResponse::Ok().body("s")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // server();
    // tcpconnect();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(upload_file)
            .service(upload_file_data)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


