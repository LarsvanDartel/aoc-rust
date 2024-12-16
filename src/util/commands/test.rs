use std::{process::Command, thread};

use crate::{AocDate, Result};

pub fn test(date: AocDate) -> Result<()> {
    let child = thread::Builder::new()
        .name(date.bin_name().to_string())
        //.stack_size(32 * 1024 * 1024)
        .spawn(move || {
            Command::new("cargo")
                .arg("test")
                .arg("--release")
                .arg("--no-fail-fast")
                .arg("--bin")
                .arg(date.bin_name())
                .arg("--")
                .arg("--nocapture")
                .arg("--test-threads=1")
                .spawn()
                .expect("Failed to spawn child process")
                .wait()
                .expect("Failed to wait on child process");
        })?;

    child.join().expect("Failed to join child thread");

    Ok(())
}
