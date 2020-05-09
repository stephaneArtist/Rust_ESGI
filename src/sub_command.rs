use std::process::{Child, Command, Stdio};

#[derive(Debug)]
pub struct SubCommand {
    pub command: Command,
    args: Vec<String>,
    pub chain: String,
    pub success: bool,
}

impl Default for SubCommand {
    fn default() -> Self {
        SubCommand {
            command: Command::new(""),
            args: Vec::with_capacity(10),
            chain: String::with_capacity(3),
            success: false,
        }
    }
}

impl SubCommand {
    pub fn new() -> SubCommand {
        SubCommand::default()
    }

    pub fn push(&mut self, arg: &str) {
        self.args.push(arg.to_string());
    }

    pub fn set_chain(&mut self, chain: &str) {
        self.chain.push_str(chain);
        self.set_command();
    }

    fn command_name(&self) -> &str {
        &self.args[0]
    }

    fn args(&self) -> &[String] {
        &self.args[1..]
    }

    pub fn set_command(&mut self) {
        let mut com = Command::new(self.command_name());
        com.args(self.args());
        self.command = com;
    }
}
