
use std::{env, fs, str::FromStr};
extern crate serde;
extern crate serde_json;
use std::env::args;
// extern crate jarvis;
// use jarvis::filemanger::filemanger::{FileManager,file_manager_section};
mod utils;
struct manager_cmd {
    manager_action: manager_actions,
    sub_action: Option<file_manager_section>,
    value: Option<String>
}
    
#[derive(Debug,PartialEq)]
enum manager_actions {
    ADD,
    REMOVE,
    RESET
    }


impl FromStr for manager_actions {
    type Err = ();
    fn from_str(input: &str) -> Result<manager_actions, Self::Err> {
        let  input = input.to_uppercase();
        match input.as_str() {
            "ADD"  => Ok(manager_actions::ADD),
            "REMOVE"  => Ok(manager_actions::REMOVE),
            "RESET"  => Ok(manager_actions::RESET),
            _      => Err(()),
        }
    }
} 

impl CLICommand for manager_cmd {
    fn run(&self) {
        println!("{:?}",&self.manager_action);
        let file_manager = FileManager::default().load();
        match self.manager_action {
            manager_actions::ADD => {
                file_manager.add(&self.sub_action.as_ref().ok_or("no").unwrap(),self.value.as_ref().ok_or("no").unwrap().as_str());
            },
            manager_actions::REMOVE => {
                file_manager.remove(&self.sub_action.as_ref().ok_or("no").unwrap(),self.value.as_ref().ok_or("no").unwrap().as_str());
            },
            manager_actions::RESET => {
                file_manager.reset();
            }
        }
    }
}
