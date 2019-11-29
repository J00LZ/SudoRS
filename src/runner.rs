use std::process::Command;
use std::os::unix::process::CommandExt;
use std::io::Error;

pub fn execute(exe: &str, args: &[&str]) -> Error {
    Command::new(exe).args(args).exec()
}