use std::io::{self};

mod command;
pub mod sub_command;
pub mod utils;

use command::ShellCommand;


fn main() -> std::io::Result<()> { 
    let stdin = io::stdin(); 
    let mut user_input = String::with_capacity(1024); 
   
    loop {
        utils::read_line("> ", &mut user_input);
        let command_input = user_input.trim();

        if !command_input.is_empty() {
            let mut shell_command = match ShellCommand::new(command_input) {
                Ok(v) => v,
                Err(e) => {
                    utils::error_message(e);
                    continue;
                }
            };   

            while !shell_command.sub_commands.is_empty() {
                let mut sub_command = match shell_command.sub_commands.pop_back() {
                    Some(v) => v,
                    None => {
                        utils::print_error();
                        break;
                    }
                };
            }
            user_input.clear();
        }
    }

    Ok(()) 
}