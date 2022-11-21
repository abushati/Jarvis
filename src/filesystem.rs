use std::{env, fs, path::PathBuf,path::Path, str::FromStr};
extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use std::env::args;
use core::fmt::Debug;


#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub path: std::path::PathBuf,
    pub file_name: String,
    pub last_accessed: std::time::SystemTime,
    pub last_modified:std::time::SystemTime,
    pub created: std::time::SystemTime,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Directory {
    pub path: std::path::PathBuf,
    pub files: Vec<File>,
    pub child_directories: Vec<Directory>,
}
