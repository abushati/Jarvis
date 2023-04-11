use std::{env, fs, str::FromStr};
extern crate serde;
extern crate serde_json;
use std::env::args;
extern crate jarvis;
use jarvis::cli::commands::{manager,sync::sync_cmd};
use jarvis::cli::commands::utils::{CliAction,CLICommand};
// use jarvis::cli::commands::utils::



enum primary_cmds {
    // MANAGER,
    STORAGE
    //  CONFIG,
    //   CLEAN
    }
impl FromStr for primary_cmds {

    type Err = ();

    fn from_str(input: &str) -> Result<primary_cmds, Self::Err> {
        let  input = input.to_uppercase();
        match input.as_str() {
            // "MANAGER" => Ok(primary_cmds::MANAGER),
            "STORAGE" => Ok(primary_cmds::STORAGE),
            // "CONFIG"  => Ok(primary_cmds::CONFIG),
            // "CLEAN"  => Ok(primary_cmds::CLEAN),
            _      => Err(()),
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
                // primary_cmds::MANAGER => {
                //     return Ok(manager::get_cmd(args).unwrap());
                    
                // },
                primary_cmds::STORAGE => {
                    let s = sync_cmd::default();
                    return  Ok(s.get_cmd(args).unwrap())
                }
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
