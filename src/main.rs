pub mod shell;
pub mod builtins;
pub mod parser;
pub mod tokeniser;
pub mod cli;
pub mod completion;

use nix::sys::signal::{signal, SigHandler, Signal};
use nix::unistd::{getpid, setpgid};
use rustyline::Result;
use crate::cli::CLI;

fn main() -> Result<()> {

    unsafe {
        //signal(Signal::SIGTTOU, SigHandler::SigIgn).ok();
        //signal(Signal::SIGTTIN, SigHandler::SigIgn).ok();
        //signal(Signal::SIGTSTP, SigHandler::SigIgn).ok();
    }

    let shell_pid = getpid();
    setpgid(shell_pid, shell_pid)?;

    let mut cli = CLI::new()?;
    cli.repl();

    Ok(())
}