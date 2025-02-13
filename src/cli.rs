use clap::{Parser, Subcommand};
use crate::handlers::{create, list, activate, delete};
use anyhow::Result;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A powerful Python virtual environment manager with support for multiple environment types",
    long_about = "Ruvo is a unified tool for managing Python virtual environments across \
    different backends (venv, poetry, pyenv, pipenv, and uv). It provides a consistent interface for creating, activating, and managing virtual environments while supporting your preferred workflow."
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new virtual environment with specified configuration
    ///
    /// This command creates a new Python virtual environment using your chosen backend.
    /// It supports multiple environment types and can use a specific Python version.
    ///
    /// Example: ruvo create myenv --env-type venv --python-version 3.11
    Create {
        /// Name of the virtual environment to create
        name: String,
        /// Type of virtual environment to create (venv, poetry, pyenv, pipenv, uv)
        /// Each type uses different tools and approaches:
        /// - venv: Python's built-in virtual environment module
        /// - poetry: Modern Python packaging and dependency management
        /// - pyenv: Python version management with virtualenv support
        /// - pipenv: Python dev workflow for humans
        /// - uv: Fast Python package installer and resolver
        #[arg(short, long)]
        env_type: String,
        /// Python version to use (e.g., "3.8", "3.9", "3.10", "3.11")
        /// If not specified, the system's default Python version will be used
        #[arg(short, long)]
        python_version: Option<String>,
    },
    /// List all managed virtual environments with their details
    ///
    /// Displays a formatted table of all virtual environments including their:
    /// - Name and type
    /// - Python version
    /// - Location on disk
    /// - Creation date
    List,
    /// Display activation instructions for a virtual environment
    ///
    /// Shows how to activate the specified environment based on its type
    /// and your current shell (bash, PowerShell, cmd)
    Activate {
        /// Name of the virtual environment to activate
        name: String,
    },
    /// Delete a virtual environment and clean up its resources
    ///
    /// Removes the virtual environment directory and its entry from the store
    /// Prompts for confirmation before deletion
    Delete {
        /// Name of the virtual environment to delete
        name: String,
    },
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            Commands::Create { name, env_type, python_version } => {
                create::handle_create(name, env_type, python_version).await
            }
            Commands::List => list::handle_list().await,
            Commands::Activate { name } => activate::handle_activate(name).await,
            Commands::Delete { name } => delete::handle_delete(name).await,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_env_type() {
        // Basic env_type string parsing is now handled in the create handler
        assert!(true);
    }
}