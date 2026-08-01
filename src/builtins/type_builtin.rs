use crate::builtins::*;
use crate::shell::get_command;

pub fn run(args: Vec<String>) -> bool {
    for arg in args {
        if is_builtin(arg.as_str()) {
            println!("{arg} is a shell builtin");
        } else {
            if let Some(cmd_path) = get_command(arg.as_str()) {
                println!("{arg} is {}", cmd_path.to_string_lossy());
            } else {
                println!("{arg}: not found");
            }
        }
    }
    true
}