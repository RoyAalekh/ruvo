use crate::core::{Store, VirtualEnv, EnvType};
use crate::error::VenvError;
use crate::utils::{check_command_exists, run_command, get_python_version, ui};
use anyhow::Result;
use std::path::PathBuf;

pub async fn handle_create(
    name: String,
    env_type: String,
    python_version: Option<String>,
) -> Result<()> {
    let mut store = Store::new()?;

    if store.get_environment(&name).is_some() {
        ui::print_error(&format!("Environment with name '{}' already exists", name));
        return Err(VenvError::EnvironmentExists(name).into());
    }

    let env_type = match env_type.as_str() {
        "venv" => EnvType::Venv,
        "poetry" => EnvType::Poetry,
        "pyenv" => EnvType::Pyenv,
        "pipenv" => EnvType::Pipenv,
        "uv" => EnvType::UV,
        _ => {
            ui::print_error(&format!("Unsupported environment type: {}", env_type));
            return Err(VenvError::UnsupportedEnvironmentType(env_type).into());
        }
    };

    // Verify required tools are installed
    match &env_type {
        EnvType::Poetry => check_tool("poetry")?,
        EnvType::Pyenv => check_tool("pyenv")?,
        EnvType::Pipenv => check_tool("pipenv")?,
        EnvType::UV => check_tool("uv")?,
        _ => (),
    }

    let spinner = ui::create_spinner(&format!("Creating {} environment '{}'...", env_type, name));

    // Create the virtual environment
    match create_environment(&name, &env_type, python_version.as_deref()) {
        Ok(path) => {
            let python_version = python_version.unwrap_or_else(|| get_python_version()
                .unwrap_or_else(|_| String::from("unknown")));

            let env = VirtualEnv::new(
                name.clone(),
                env_type,
                python_version,
                path,
            );

            store.add_environment(env)?;
            spinner.finish_and_clear();
            ui::print_success(&format!("Virtual environment '{}' created successfully!", name));
            Ok(())
        }
        Err(e) => {
            spinner.finish_and_clear();
            ui::print_error(&format!("Failed to create environment: {}", e));
            Err(e)
        }
    }
}

fn check_tool(tool: &str) -> Result<()> {
    if !check_command_exists(tool) {
        ui::print_error(&format!("{} is not installed", tool));
        return Err(VenvError::CommandError(format!("{} is not installed", tool)).into());
    }
    Ok(())
}

fn create_environment(
    name: &str,
    env_type: &EnvType,
    python_version: Option<&str>,
) -> Result<PathBuf> {
    let path = std::env::current_dir()?.join(name);

    match env_type {
        EnvType::Venv => create_venv(name)?,
        EnvType::Poetry => create_poetry(name, &path, python_version)?,
        EnvType::Pyenv => create_pyenv(name, python_version)?,
        EnvType::Pipenv => create_pipenv(&path)?,
        EnvType::UV => create_uv(&path)?,
    }

    Ok(path)
}

fn create_venv(name: &str) -> Result<()> {
    run_command("python", &["-m", "venv", name], None)
}

fn create_poetry(name: &str, path: &PathBuf, python_version: Option<&str>) -> Result<()> {
    std::fs::create_dir_all(path)?;

    let mut args = vec!["init", "--name", name, "--no-interaction"];
    if let Some(version) = python_version {
        args.extend_from_slice(&["--python", version]);
    }

    run_command("poetry", &args, Some(path))
}

fn create_pyenv(name: &str, python_version: Option<&str>) -> Result<()> {
    if let Some(version) = python_version {
        run_command("pyenv", &["install", version], None)?;
    }
    run_command("pyenv", &["virtualenv", name], None)
}

fn create_pipenv(path: &PathBuf) -> Result<()> {
    std::fs::create_dir_all(path)?;
    run_command("pipenv", &["--python", "3"], Some(path))
}

fn create_uv(path: &PathBuf) -> Result<()> {
    std::fs::create_dir_all(path)?;
    run_command("uv", &["venv", "create"], Some(path))
}