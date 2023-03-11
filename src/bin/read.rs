use bytes::Bytes;
use std::io::Read;
use std::{fs::OpenOptions, io::Write};
use actix_multipart::Multipart;
use serde::Deserialize;
use actix_web::{get, post, web, App,HttpRequest, HttpResponse, HttpServer, Responder};
extern crate redis;
use redis::Commands;
use redis::{Value, FromRedisValue,RedisError};
use std::collections::HashMap;
use uuid::Uuid;
use md5;
use std::str;
use std::{thread, time::Duration};

fn main()  {
    loop {
        let client = redis::Client::open("redis://localhost:6379").unwrap();
        let mut con = client.get_connection().unwrap();
        let key = "upload_queue";
        let data : String = con.lpop(key,None).unwrap();
        println!("{:?}",data);
        thread::sleep(Duration::from_secs(4));
    }
}
