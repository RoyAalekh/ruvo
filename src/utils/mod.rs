use std::process::Command;
use anyhow::Result;

pub mod ui;

pub fn check_command_exists(command: &str) -> bool {
    which::which(command).is_ok()
}

pub fn run_command(command: &str, args: &[&str], dir: Option<&std::path::Path>) -> Result<()> {
    let mut cmd = Command::new(command);
    if let Some(path) = dir {
        cmd.current_dir(path);
    }
    cmd.args(args)
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;
    Ok(())
}

pub fn get_python_version() -> Result<String> {
    let output = Command::new("python")
        .args(&["--version"])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to get Python version: {}", e))?;

    let version = String::from_utf8(output.stdout)
        .map_err(|e| anyhow::anyhow!("Invalid Python version output: {}", e))?;

    Ok(version.trim().to_string())
}