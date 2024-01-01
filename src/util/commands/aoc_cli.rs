use std::{
    path::PathBuf,
    process::{Command, Output},
};

use crate::{AocDate, Result, CARGO_ROOT};

#[derive(Debug)]
pub enum AocClientError {
    CommandNotFound,
    BadExitStatus(Output),
}

impl std::fmt::Display for AocClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocClientError::CommandNotFound => write!(
                f,
                "aoc-cli not found, use `cargo install aoc-cli` to install it"
            ),
            AocClientError::BadExitStatus(_) => write!(f, "aoc-cli exited with a non-zero status"),
        }
    }
}

pub fn check_aoc_cli() -> Result<()> {
    Command::new("aoc")
        .arg("--version")
        .output()
        .map_err(|_| AocClientError::CommandNotFound)?;

    Ok(())
}

pub fn download_input(date: &AocDate) -> Result<()> {
    let path = date.input_path()?;
    if path.exists() {
        println!(
            "🎄 Input for {} day {} already exists at {}",
            date.year,
            date.day,
            path.strip_prefix(CARGO_ROOT).unwrap().display()
        );
        return Ok(());
    }

    let mut session_path = PathBuf::from(CARGO_ROOT);
    session_path.push(".session");

    let output = Command::new("aoc")
        .arg("download")
        .arg("--input-only")
        .arg("--input")
        .arg(&path)
        .arg("--year")
        .arg(date.year.to_string())
        .arg("--day")
        .arg(date.day.to_string())
        .arg("--session-file")
        .arg(session_path)
        .output()
        .map_err(|_| AocClientError::CommandNotFound)?;

    if !output.status.success() {
        Err(AocClientError::BadExitStatus(output))?;
    }

    println!(
        "🎄 Downloaded input for {} day {} to {}",
        date.year,
        date.day,
        path.strip_prefix(CARGO_ROOT).unwrap().display()
    );
    Ok(())
}
