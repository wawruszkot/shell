use std::env;
use std::ffi::{CString, OsString};
use std::fs;
use std::os::fd::{AsRawFd, BorrowedFd};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use nix::fcntl::{open, OFlag};
use nix::libc::{self, STDERR_FILENO, STDOUT_FILENO};
use nix::sys::stat::Mode;
use nix::sys::wait::waitpid;
use nix::unistd::{close, dup, execv, fork, getpid, setpgid, tcsetpgrp, ForkResult};

use crate::builtins;
use crate::parser::{self, Redirection};

pub fn execute(command: parser::Command) -> bool {
    let stdin = unsafe {
        BorrowedFd::borrow_raw(0)
    };

    if let Some(name) = command.cmd {
        if let Some(builtin_cmd) = builtins::BUILTINS.iter().find(|x| x.name == name) {
            let backup_out = dup(unsafe { BorrowedFd::borrow_raw(STDOUT_FILENO) }).unwrap();
            let backup_err = dup(unsafe { BorrowedFd::borrow_raw(STDERR_FILENO) }).unwrap();

            for redirection in command.redirections {
                handle_redirection(redirection).unwrap();
            }

            let res = (builtin_cmd.run)(command.args);

            unsafe {
                libc::dup2(backup_out.as_raw_fd(), STDOUT_FILENO);
                libc::dup2(backup_err.as_raw_fd(), STDERR_FILENO);
            }
            return res;
        }
        else {
            match unsafe {fork()} {
                Ok(ForkResult::Parent {child, ..}) => {
                    setpgid(child, child).unwrap();
                    tcsetpgrp(stdin, child).unwrap();
                    waitpid(child, None).unwrap();
                    tcsetpgrp(stdin, getpid()).unwrap();
                }
                Ok(ForkResult::Child) => {
                    setpgid(getpid(), getpid()).unwrap();
                    for redirection in command.redirections {
                        if handle_redirection(redirection).is_err() {
                            std::process::exit(1);
                        }
                    }

                    if let Some(external_cmd) = get_command(name.as_str()) {
                        handle_external_cmd(name.as_str(), external_cmd, command.args).unwrap();
                    } else {
                        println!("{}: command not found", name);
                        std::process::exit(127);
                    }
                }
                Err(_) => eprintln!("Failed to fork process"),
            }
        }
    }
    true
}

fn is_executable(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => {
            metadata.is_file() &&
                (metadata.permissions().mode() & 0o111 != 0)
        }
        Err(_) => false,
    }
}

pub fn get_command(name: &str) -> Option<OsString> {
    let paths = env::var_os("PATH")?;

    for dir in env::split_paths(&paths) {
        let candidate = dir.join(name);
        if is_executable(&candidate) {
            return Some(candidate.into_os_string())
        }
    }
    None
}

fn handle_redirection (redirection: Redirection) -> nix::Result<()> {
    let (flags, target_fd, path) = match redirection {
        Redirection::Output(p) => (OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC, STDOUT_FILENO, p),
        Redirection::OutputAppend(p) => (OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND, STDOUT_FILENO, p),
        Redirection::Error(p) => (OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC, STDERR_FILENO, p),
        Redirection::ErrorAppend(p) => (OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND, STDERR_FILENO, p),
    };

    let fd = open(
        path.as_str(),
        flags,
        Mode::from_bits_truncate(0o666),
    )?;

    let result = unsafe { libc::dup2(fd.as_raw_fd(), target_fd) };

    close(fd)?;

    if result == -1 {
        return Err(nix::Error::last());
    }

    Ok(())
}

fn handle_external_cmd (cmd_name: &str, cmd: OsString, args: Vec<String>) -> nix::Result<()> {
    let cmd = CString::new(cmd.as_os_str().as_bytes()).unwrap();
    let mut argv = Vec::with_capacity(args.len() + 1);
    argv.push(CString::new(cmd_name).unwrap());

    for arg in args {
        argv.push(CString::new(arg).unwrap());
    }

    execv(&cmd, &argv)?;
    Ok(())
}