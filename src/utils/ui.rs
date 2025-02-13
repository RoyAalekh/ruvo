use colored::*;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

// Removed unused print_warning function

pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

pub fn print_environment_info(
    name: &str,
    env_type: &str,
    python_version: &str,
    path: &str,
    created_at: &str,
) {
    println!("{}", "─".repeat(40).bright_black());
    println!("{}: {}", style("Name").cyan().bold(), name);
    println!("{}: {}", style("Type").cyan().bold(), env_type);
    println!("{}: {}", style("Python Version").cyan().bold(), python_version);
    println!("{}: {}", style("Location").cyan().bold(), path);
    println!("{}: {}", style("Created").cyan().bold(), created_at);
    println!("{}", "─".repeat(40).bright_black());
}

pub fn print_logo() {
    let logo = r#"
    🐍 Ruvo 🐍
    =============================
    "#;
    println!("{}", style(logo).cyan().bold());
}