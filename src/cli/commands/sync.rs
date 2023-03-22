use super::utils::{CLICommand,CliAction};
use std::fs::{read_dir,read,OpenOptions,write};
use std::io::Write;
use std::path;
use std::str::from_utf8;
#[derive(Default)]


//Todo: add enum for type
pub struct sync_cmd {
    type_arg: String,
    path: String
}
impl CLICommand for sync_cmd {
    fn run(&self) {
        // let i = vec!["d","ds"].contains(x);
        let e = format!("Hello from run of sync type {:?}, path: {:?}",&self.type_arg, &self.path);
        println!("{}",e);
        if ["d","directory"].contains(&self.type_arg.as_str()) {
            let path = path::Path::new(&self.path);
            let i = read_dir(path);
            match i {
                Ok(..) => {
                    for s in i.unwrap() {
                        let entry = s.unwrap();
                        let file_name = entry.file_name();
                        let file_path = entry.path();
                        let file_meta = entry.metadata().unwrap();
                        let file_bytes = read(&file_path).unwrap();
                        // write("Users/arvidbushati/Desktop/Projects/Jarvis/here.txt",&file_bytes).unwrap();

                        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
                        .open("/Users/arvidbushati/Desktop/Projects/Jarvis/here.txt").unwrap();
                        file.write_all(&file_bytes).unwrap();
                        println!("{:?},{:?},{:?},{:?}",file_name,&file_path,file_meta,file_bytes);
        
                    }
                }
                Err(e) => {println!("what is u doing? {:?}",e)}
            }
            
        }
        
    }
    fn get_cmd (&self, args: Vec<String>) -> Result<CliAction,String> {
        let type_arg = args.get(2).unwrap().to_string();
        let path = args.get(3).unwrap().to_string();
        let cmd = sync_cmd{type_arg:type_arg,
                                    path:path};
        Ok(CliAction{cmd:Box::new(cmd)})
    
    }
}

