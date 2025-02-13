use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VirtualEnv {
    pub name: String,
    pub env_type: EnvType,
    pub python_version: String,
    pub path: PathBuf,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EnvType {
    Venv,
    Poetry,
    Pyenv,
    Pipenv,
    UV,
}

impl std::fmt::Display for EnvType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvType::Venv => write!(f, "venv (Python's built-in virtual environment)"),
            EnvType::Poetry => write!(f, "Poetry (Modern Python packaging tool)"),
            EnvType::Pyenv => write!(f, "Pyenv (Python version manager)"),
            EnvType::Pipenv => write!(f, "Pipenv (Python development workflow)"),
            EnvType::UV => write!(f, "UV (Fast Python package installer)"),
        }
    }
}

impl VirtualEnv {
    pub fn new(
        name: String,
        env_type: EnvType,
        python_version: String,
        path: PathBuf,
    ) -> Self {
        Self {
            name,
            env_type,
            python_version,
            path,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn get_type_description(&self) -> &'static str {
        match self.env_type {
            EnvType::Venv => "Python's built-in virtual environment module",
            EnvType::Poetry => "Modern Python packaging and dependency management",
            EnvType::Pyenv => "Python version management tool",
            EnvType::Pipenv => "Python development workflow tool",
            EnvType::UV => "Fast Python package installer and resolver",
        }
    }
}