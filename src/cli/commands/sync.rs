use super::utils::{CLICommand,CliAction};

#[derive(Default)]
pub struct sync_cmd {
    file: String
}
impl CLICommand for sync_cmd {
    fn run(&self) {
        let e = format!("Hello from run of sync {:?}",&self.file);
        println!("{}",e);
    }
    fn get_cmd (&self, args: Vec<String>) -> Result<CliAction,String> {
        let cmd = sync_cmd{file:args.get(2).unwrap().to_string()};
        Ok(CliAction{cmd:Box::new(cmd)})
    
    }
}
