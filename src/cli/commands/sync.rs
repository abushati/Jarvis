use super::utils::{CLICommand,CliAction};
use crate::syner::syncer;
use std::str::FromStr;

/*
You can use the magic crate in Rust to determine the file type using the magic number. Here is an example:

rust
Copy code
use magic::flags::{MIME_TYPE, MIME_ENCODING};
use magic::{Cookie, CookieFlags};

fn is_image(file_path: &str) -> bool {
    let cookie_flags = CookieFlags::MIME_TYPE | CookieFlags::MIME_ENCODING;
    let cookie = Cookie::open(cookie_flags).unwrap();
    let mime_type = cookie
        .get_mime_type(file_path)
        .unwrap_or("application/octet-stream".to_string());
    mime_type.starts_with("image/")
}
This code uses the magic::Cookie struct to open a new magic cookie with the MIME_TYPE and MIME_ENCODING flags set. The get_mime_type method of the cookie is then used to get the MIME type of the file, which is checked to see if it starts with the "image/" prefix. If it does, then the file is an image.

Note that the magic crate requires the installation of the libmagic library on your system.


 */

//Todo: add enum for type
#[derive(Default)]
enum StorageCommands {
    Upload,
    Delete,
    Update,
    #[default] Read,
}
impl FromStr for StorageCommands {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "upload" => Ok(StorageCommands::Upload),
            "delete" => Ok(StorageCommands::Delete),
            "update" => Ok(StorageCommands::Update),
            "read" => Ok(StorageCommands::Read),
            _ => Err("bad".to_string())
        }
    }

    
}

#[derive(Default)]
pub struct StorageCommand {
    storage_action: StorageCommands,
    type_arg: String,
    path: String
}
impl CLICommand for StorageCommand {
    fn run(&self) {
        let e = format!("Hello from run of sync type {:?}, path: {:?}",&self.type_arg, &self.path);
        println!("{}",e);
        let syncer = syncer::new();
        match self.storage_action {
            StorageCommands::Upload => {
                match self.type_arg.as_str() {
                    "directory" | "d" => {
                        syncer.upload_directory(&self.path)
                    },
                    "file" | "f" => {
                        syncer.upload_file(&self.path)
                    },
                    _ => {
                        println!("Bad sync type");
                    }
                }
            },
            StorageCommands::Read => {
                match self.type_arg.as_str() {
                    "file" | "f" => {
                        syncer.read_file(&self.path)
                    },
                    _ => {
                        println!("invalid");
                    }
                }
            },
            StorageCommands::Delete => {
                match self.type_arg.as_str() {
                    "file" | "f" => {
                        syncer.delete_file(&self.path);
                    },
                    _ => {
                        println!("invalid");
                    }
                }
            },        
            _ => {
                unimplemented!();
            }
        }
    }
    
    fn get_cmd (&self, args: Vec<String>) -> Result<CliAction,String> {

        let storage_action = args.get(2).unwrap();
        let type_arg = args.get(3).unwrap().to_string();
        let path = args.get(4).unwrap().to_string();
        let cmd = StorageCommand{storage_action: StorageCommands::from_str(storage_action).unwrap(),
                                    type_arg:type_arg,
                                    path:path};
        Ok(CliAction{cmd:Box::new(cmd)})
    
    }
}

