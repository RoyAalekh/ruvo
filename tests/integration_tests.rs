use anyhow::Result;
use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::path::PathBuf;
use std::env;

// Helper function to set up test environment
fn setup_test_env() -> PathBuf {
    let test_dir = env::temp_dir().join(format!("ruvo-test-{}", rand::random::<u32>()));
    env::set_var("RUVO_STORE_PATH", test_dir.join("store.json").to_str().unwrap());
    test_dir
}

#[tokio::test]
async fn test_error_handling() -> Result<()> {
    let test_dir = setup_test_env();

    // Test invalid environment type
    let mut cmd = Command::cargo_bin("ruvo")?;
    cmd.args(&["create", "test-env", "--env-type", "invalid"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported environment type"));

    // Test activating non-existent environment
    let mut cmd = Command::cargo_bin("ruvo")?;
    cmd.args(&["activate", "non-existent"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found"));

    // Test deleting non-existent environment
    let mut cmd = Command::cargo_bin("ruvo")?;
    cmd.args(&["delete", "non-existent"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found"));

    // Cleanup test directory
    if test_dir.exists() {
        std::fs::remove_dir_all(test_dir)?;
    }

    Ok(())
}