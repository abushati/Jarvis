use std::{net::{TcpListener, TcpStream}, io::Read};
use std::str::from_utf8;
use std::process::Command;

fn handle_client(mut stream: TcpStream) {
    let mut t:Vec<u8> = vec![];
    let i = stream.read_to_end(&mut t);
    let q = i.unwrap().to_string();
    println!("{:?}",q);
    let cmd = from_utf8(&t).unwrap();
    println!("{:?}",&cmd);

    let mut output = Command::new(cmd);
    output.status().expect("process failed to execute");

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