trait  CLICommand {
    fn run(&self){}
}

struct CliAction {
    cmd: Box<dyn CLICommand>,
}