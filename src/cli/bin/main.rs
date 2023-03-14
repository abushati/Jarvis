use std::{env, fs, str::FromStr};
extern crate serde;
extern crate serde_json;
use std::env::args;
extern crate jarvis;
use jarvis::filemanger::filemanger::{FileManager,file_manager_section};

struct CliAction {
    cmd: Box<dyn CLICommand>,
}
trait  CLICommand {
    fn run(&self){}
}
enum primary_cmds {
    MANAGER,
    //  CONFIG,
    //   CLEAN
    }
impl FromStr for primary_cmds {

    type Err = ();

    fn from_str(input: &str) -> Result<primary_cmds, Self::Err> {
        let  input = input.to_uppercase();
        match input.as_str() {
            "MANAGER"  => Ok(primary_cmds::MANAGER),
            // "CONFIG"  => Ok(primary_cmds::CONFIG),
            // "CLEAN"  => Ok(primary_cmds::CLEAN),
            _      => Err(()),
        }
    }
}


struct manager_cmd{
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

fn parse_args() -> Result<CliAction,String> {
    // {"manager":{"add":["action","type"],
    //             "remove":["action","type"],
    //             "reset": []
    //              },
    //"config":["action","key","value"]
    //         
    // "clean":{}}
    let args:Vec<String> = args().collect();
    if !(args.len() > 1){
        return Err(String::from_str("invalid args len").unwrap());
    }
    
    let primary_cmd = args.get(1).unwrap();
    match primary_cmds::from_str(&primary_cmd){
        Ok(act) => {
            match act {
                primary_cmds::MANAGER => {
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
                },
                // primary_cmds::CONFIG => {return Err()},
                // primary_cmds::CLEAN => {return Err()}

            }

        }
        Err(()) => {return Err(format!("invalid primary arg {}, valid args: {:?}", "primary_cmd", "valid_primary_cmds"));}
    } 
}

pub fn main() {
    let db = parse_args();
    match db {
        Ok(action) => {
            let cmd = action.cmd;
            cmd.run()
            // action.run_action();
        },
        Err(error) =>{
            println!("{}", error)
        }
        
    }
}
