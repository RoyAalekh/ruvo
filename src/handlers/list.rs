use crate::core::Store;
use crate::utils::ui;
use anyhow::Result;
use chrono::prelude::*;

pub async fn handle_list() -> Result<()> {
    let store = Store::new()?;
    let environments = store.list_environments();

    if environments.is_empty() {
        ui::print_info("No virtual environments found");
        return Ok(());
    }

    ui::print_logo();
    ui::print_info("Managed Virtual Environments:");

    for env in environments {
        ui::print_environment_info(
            &env.name,
            &format!("{} - {}", env.env_type, env.get_type_description()),
            &env.python_version,
            &env.path.display().to_string(),
            &env.created_at.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
        );
    }

    Ok(())
}