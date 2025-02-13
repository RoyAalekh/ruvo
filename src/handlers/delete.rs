use crate::core::Store;
use crate::error::VenvError;
use crate::utils::ui;
use anyhow::Result;
use std::fs;
use dialoguer::Confirm;
use std::env;

pub async fn handle_delete(name: String) -> Result<()> {
    let mut store = Store::new()?;

    let env = store.get_environment(&name)
        .ok_or_else(|| {
            ui::print_error(&format!("Environment '{}' not found", name));
            VenvError::EnvironmentNotFound(name.clone())
        })?;

    // Skip confirmation in test mode
    let should_delete = env::var("RUVO_TEST_MODE").is_ok() ||
        Confirm::new()
            .with_prompt(format!("Are you sure you want to delete environment '{}'?", name))
            .default(false)
            .interact()?;

    if !should_delete {
        ui::print_info("Deletion cancelled");
        return Ok(());
    }

    let spinner = ui::create_spinner(&format!("Deleting environment '{}'...", name));

    // Remove the environment directory
    if env.path.exists() {
        if let Err(e) = fs::remove_dir_all(&env.path) {
            spinner.finish_and_clear();
            ui::print_error(&format!("Failed to delete directory: {}", e));
            return Err(VenvError::CreationError(format!("Failed to delete directory: {}", e)).into());
        }
    }

    // Remove from store
    if let Err(e) = store.remove_environment(&name) {
        spinner.finish_and_clear();
        ui::print_error(&format!("Failed to remove from store: {}", e));
        return Err(e);
    }

    spinner.finish_and_clear();
    ui::print_success(&format!("Environment '{}' deleted successfully", name));
    Ok(())
}