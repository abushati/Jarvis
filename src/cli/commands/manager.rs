use std::{str::FromStr};
extern crate serde;
extern crate serde_json;

use crate::filemanger::filemanger::{file_manager_section,FileManager};
use super::utils::{CLICommand,CliAction};

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

pub fn get_cmd(args: Vec<String>) -> Result<CliAction,String>{

    let manager_action;
    let manager_section;
    let value;

    if args.get(2).is_none(){
        return Err(format!("No sub action provided, valid args: {:?}" , "adf"));
    }
    match manager_actions::from_str(args.get(2).unwrap()){
        Ok(action) => {
            manager_action = action;
        }
        Err(()) => {
            return Err(format!("Invalid manager action" ));
        }
    }
    
    if manager_action == manager_actions::RESET {
        let cmd = manager_cmd{manager_action:manager_action, sub_action: None, value: None};
        return Ok(CliAction{cmd:Box::new(cmd)});
    }

    let section = args.get(3).unwrap();
    match file_manager_section::from_str(&section.as_str()) {
        Ok(section) => {
            manager_section = section
        }
        Err(()) => {
            return Err(format!("Invalid subaction {}, valid args: {:?}" ,"adf", "adf"));
        }
    }
    
    let path = args.get(4);
    if !path.is_none(){
        value = path.unwrap();

    } else {
        return Err(format!("path not provided"));
    }

    let cmd = manager_cmd{manager_action:manager_action,sub_action:Some(manager_section),value:Some(value.to_string())};
    Ok(CliAction{cmd:Box::new(cmd)})


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
