use crate::sub_command::SubCommand;
use crate::utils;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct ShellCommand {
    pub sub_commands: LinkedList<SubCommand>,
}

impl ShellCommand {
    pub fn new(input: &str) -> Result<ShellCommand, String> {
        let mut self_ = ShellCommand {
            sub_commands: LinkedList::new(),
        };

        let mut sub_command = SubCommand::new();

        Ok(self_)
    }
}
