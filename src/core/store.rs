use crate::error::VenvError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    environments: HashMap<String, super::VirtualEnv>,
    #[serde(skip)]
    store_path: PathBuf,
}

impl Store {
    pub fn new() -> Result<Self> {
        let store_path = if let Ok(test_path) = env::var("RUVO_STORE_PATH") {
            PathBuf::from(test_path)
        } else {
            get_store_path()?
        };
        Self::with_path(store_path)
    }

    pub fn with_path(store_path: PathBuf) -> Result<Self> {
        // Create the parent directory if it doesn't exist
        if let Some(parent) = store_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let store = if store_path.exists() {
            let content = fs::read_to_string(&store_path)
                .map_err(|e| VenvError::StoreError(format!("Failed to read store file: {}", e)))?;
            let mut store: Store = serde_json::from_str(&content)
                .map_err(|e| VenvError::StoreError(format!("Failed to parse store data: {}", e)))?;
            store.store_path = store_path;
            store
        } else {
            Store {
                environments: HashMap::new(),
                store_path,
            }
        };
        Ok(store)
    }

    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| VenvError::StoreError(format!("Failed to serialize store data: {}", e)))?;
        fs::write(&self.store_path, content)
            .map_err(|e| VenvError::StoreError(format!("Failed to write store file: {}", e)).into())
    }

    pub fn add_environment(&mut self, env: super::VirtualEnv) -> Result<()> {
        if self.environments.contains_key(&env.name) {
            return Err(VenvError::EnvironmentExists(env.name).into());
        }
        self.environments.insert(env.name.clone(), env);
        self.save()
    }

    pub fn remove_environment(&mut self, name: &str) -> Result<()> {
        if !self.environments.contains_key(name) {
            return Err(VenvError::EnvironmentNotFound(name.to_string()).into());
        }
        self.environments.remove(name);
        self.save()
    }

    pub fn get_environment(&self, name: &str) -> Option<&super::VirtualEnv> {
        self.environments.get(name)
    }

    pub fn list_environments(&self) -> Vec<&super::VirtualEnv> {
        self.environments.values().collect()
    }
}

fn get_store_path() -> Result<PathBuf> {
    let proj_dirs = directories::ProjectDirs::from("com", "ruvo", "store")
        .ok_or_else(|| VenvError::StoreError("Failed to get project directories".to_string()))?;
    let data_dir = proj_dirs.data_dir();
    fs::create_dir_all(data_dir)
        .map_err(|e| VenvError::StoreError(format!("Failed to create data directory: {}", e)))?;
    Ok(data_dir.join("store.json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{EnvType, VirtualEnv};
    use tempfile::tempdir;

    #[test]
    fn test_store_operations() -> Result<()> {
        // Create a temporary directory for the test store
        let temp_dir = tempdir()?;
        let store_path = temp_dir.path().join("test-store.json");
        let mut store = Store::with_path(store_path)?;

        // Test adding environment
        let env = VirtualEnv::new(
            "test-env".to_string(),
            EnvType::Venv,
            "3.11".to_string(),
            PathBuf::from("/tmp/test-env"),
        );
        store.add_environment(env.clone())?;
        assert_eq!(store.list_environments().len(), 1);

        // Test getting environment
        let stored_env = store.get_environment("test-env").unwrap();
        assert_eq!(stored_env.name, "test-env");
        assert_eq!(stored_env.python_version, "3.11");

        // Test removing environment
        store.remove_environment("test-env")?;
        assert_eq!(store.list_environments().len(), 0);

        // Test error on duplicate
        store.add_environment(env.clone())?;
        let duplicate_result = store.add_environment(env);
        assert!(matches!(duplicate_result.unwrap_err().downcast::<VenvError>().unwrap(),
                        VenvError::EnvironmentExists(_)));

        // Test error on removing non-existent
        let not_found_result = store.remove_environment("non-existent");
        assert!(matches!(not_found_result.unwrap_err().downcast::<VenvError>().unwrap(),
                        VenvError::EnvironmentNotFound(_)));

        Ok(())
    }
}