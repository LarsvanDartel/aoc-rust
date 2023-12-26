use std::process::Command;

use crate::{AocDate, Result};

pub fn solve(date: AocDate, _submit: Option<u8>) -> Result<()> {
    Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(&date.bin_name())
        .arg("--")
        .arg(&date.input_path())
        .spawn()?
        .wait()?;
    Ok(())
}
