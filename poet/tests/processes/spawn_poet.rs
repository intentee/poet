use std::io;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;

pub fn spawn_poet(arguments: &[&str]) -> io::Result<Child> {
    Command::new(env!("CARGO_BIN_EXE_poet"))
        .args(arguments)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}
