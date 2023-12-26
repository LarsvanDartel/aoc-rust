use std::{process::Command, thread};

use crate::{AocDate, Result};

pub fn solve(date: AocDate, _submit: Option<u8>) -> Result<()> {
    let child = thread::Builder::new()
        .name(date.bin_name().to_string())
        //.stack_size(32 * 1024 * 1024)
        .spawn(move || {
            Command::new("cargo")
                .arg("run")
                .arg("--bin")
                .arg(&date.bin_name())
                .arg("--")
                .arg(&date.input_path())
                .spawn()
                .expect("Failed to spawn child process")
                .wait()
                .expect("Failed to wait on child process");
    })?;

    child.join().expect("Failed to join child thread");

    Ok(())
}
