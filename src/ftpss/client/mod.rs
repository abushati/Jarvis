// pub mod ftpclient;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use {
    chrono::{offset::TimeZone, DateTime, Utc},
    // regex::Regex,
    std::{
        borrow::Cow,
        io::{copy, BufRead, BufWriter, Cursor, Read, Write},
        net::{SocketAddr, TcpStream, ToSocketAddrs},
        str::FromStr,
    },
};
pub struct test {}

pub fn tcpconnect() {
    let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    // let msg = "this is a message {}";
    // let mut num = 0;
    // loop {
    //     let u = format!("this is a message {}",num);
    //     stream.write(u.as_bytes());
    //     num += 1;
    // }
    let mut buffer = vec![];
    let mut t = File::open("/Users/arvidbushati/Desktop/Projects/Jarvis/test.docx").unwrap();
    
    println!("{:?}", t);
    t.read_to_end(&mut buffer);
    println!("{:?}",buffer);
    stream.write(&buffer);
    
}