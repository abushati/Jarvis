use super::utils::{CLICommand,CliAction};

struct sync_cmd {
    file: String
}
impl CLICommand for sync_cmd {
    fn run(&self) {
        let e = format!("Hello from run of sync {:?}",&self.file);
        println!("{}",e);
    }
}

pub fn get_cmd (args: Vec<String>) -> Result<CliAction,String> {
    let cmd = sync_cmd{file:args.get(2).unwrap().to_string()};
    Ok(CliAction{cmd:Box::new(cmd)})

}