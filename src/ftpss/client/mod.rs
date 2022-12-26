// pub mod ftpclient;

use {
    chrono::{offset::TimeZone, DateTime, Utc},
    // regex::Regex,
    std::{
        borrow::Cow,
        io::{copy, BufRead, BufReader, BufWriter, Cursor, Read, Write},
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

    let cmd = "ls";
    stream.write(cmd.as_bytes());
    // stream.read(&mut [0; 128]);
    // Ok(())
}