use std::{net::{TcpListener, TcpStream}, io::Read};
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) {
    let mut t:Vec<u8> = vec![];
    let i = stream.read_to_end(&mut t);
    let q = i.unwrap().to_string();
    println!("{:?}",q);
    let mut f = File::create("/Users/arvidbushati/Desktop/tst.file").unwrap();
    f.write_all(&t);

    println!();
}

pub fn server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:34254")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}