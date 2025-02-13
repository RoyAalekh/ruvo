use crate::core::Store;
use crate::error::VenvError;
use crate::utils::ui;
use anyhow::Result;

pub async fn handle_activate(name: String) -> Result<()> {
    let store = Store::new()?;
    let env = store.get_environment(&name)
        .ok_or_else(|| {
            ui::print_error(&format!("Environment '{}' not found", name));
            VenvError::EnvironmentNotFound(name.clone())
        })?;

    ui::print_info(&format!("Activation instructions for '{}':", name));
    println!();

    match env.env_type {
        crate::core::EnvType::Venv => {
            println!("# For Unix-like systems:");
            println!("source {}/bin/activate", env.path.display());
            println!("\n# For Windows (PowerShell):");
            println!("{}/Scripts/Activate.ps1", env.path.display());
            println!("\n# For Windows (Command Prompt):");
            println!("{}/Scripts/activate.bat", env.path.display());
        }
        crate::core::EnvType::Poetry => {
            println!("cd {} && poetry shell", env.path.display());
        }
        crate::core::EnvType::Pyenv => {
            println!("pyenv activate {}", name);
        }
        crate::core::EnvType::Pipenv => {
            println!("cd {} && pipenv shell", env.path.display());
        }
        crate::core::EnvType::UV => {
            println!("# For Unix-like systems:");
            println!("source {}/bin/activate", env.path.display());
            println!("\n# For Windows:");
            println!("{}/Scripts/activate", env.path.display());
        }
    }
    println!();
    ui::print_info("Copy and run the appropriate command to activate the environment");
    Ok(())
}