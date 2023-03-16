pub trait CLICommand {
    fn run(&self);
    fn get_cmd (&self, args: Vec<String>) -> Result<CliAction,String>;
}

pub struct CliAction {
    pub cmd: Box<dyn CLICommand>,
}