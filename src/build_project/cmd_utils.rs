use std::process::{self, Command};

pub(crate) fn spawn_and_wait(mut cmd: Command) {
    let status = cmd.spawn().unwrap().wait().unwrap();
    if !status.success() {
        eprintln!("{cmd:?} exited with status {:?}", status);
        process::exit(1);
    }
}
