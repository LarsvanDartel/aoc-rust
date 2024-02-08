use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use toml::{map::Map, Value};

use super::aoc_cli::{check_aoc_cli, download_input};
use crate::{AocDate, Result, CARGO_ROOT};

pub fn scaffold(date: AocDate) -> Result<()> {
    check_aoc_cli()?;
    download_input(&date)?;
    create_bin(&date)?;
    add_cargo_bin(&date)?;
    open_editor(date.bin_path()?)?;
    open_editor(date.input_path()?)?;
    Ok(())
}

fn open_editor(path: PathBuf) -> Result<()> {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "code.cmd".into());
    let status = Command::new(editor).arg(path).status()?;
    if !status.success() {
        Err("Failed to open editor")?;
    }
    Ok(())
}

fn create_bin(date: &AocDate) -> Result<()> {
    let path = date.bin_path()?;

    if path.exists() {
        println!(
            "🎄 Template for {} day {} already exists at {}",
            date.year,
            date.day,
            path.strip_prefix(CARGO_ROOT).unwrap().display()
        );
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&path)?;

    let mut template_path = PathBuf::from(CARGO_ROOT);
    template_path.push("src");
    template_path.push("templates");
    template_path.push("day");
    template_path.set_extension("txt");

    let mut template = std::fs::read_to_string(template_path)?;
    template = template.replace("{{year}}", format!("{:0>4}", date.year).as_str());
    template = template.replace("{{day}}", format!("{:0>2}", date.day).as_str());

    file.write_all(template.as_bytes())?;
    println!(
        "🎄 Created template for {} day {} at {}",
        date.year,
        date.day,
        path.strip_prefix(CARGO_ROOT).unwrap().display()
    );
    Ok(())
}

fn add_cargo_bin(date: &AocDate) -> Result<()> {
    let mut path = PathBuf::from(CARGO_ROOT);
    path.push("Cargo.toml");
    let cargo_toml = std::fs::read_to_string(&path)?;
    let mut cargo_toml: Value = toml::from_str(&cargo_toml)?;
    let bin_name = date.bin_name();
    let bin_path = date
        .bin_path()?
        .strip_prefix(CARGO_ROOT)?
        .display()
        .to_string()
        .replace('\\', "/");

    let bins = cargo_toml
        .get_mut("bin")
        .and_then(|v| v.as_array_mut())
        .ok_or("No bins configured? You changed and broke something.")?;
    if !bins
        .iter()
        .any(|v| v.get("name").and_then(|v| v.as_str()) == Some(&bin_name))
    {
        bins.push(Value::Table(Map::from_iter([
            ("name".into(), Value::String(bin_name.clone())),
            ("path".into(), Value::String(bin_path.clone())),
        ])))
    }
    let mut cargo_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .truncate(true)
        .open(path)?;

    cargo_file.write_all(toml::to_string_pretty(&cargo_toml)?.as_bytes())?;
    println!(
        "🎄 Added {} to Cargo.toml, you can now run it with `cargo solve`",
        bin_name
    );
    Ok(())
}
