pub trait  CLICommand {
    fn run(&self){}
}

pub struct CliAction {
    pub cmd: Box<dyn CLICommand>,
}