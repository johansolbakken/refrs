use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    absolute_path: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppState {
    projects: Vec<Project>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
        }
    }
}

#[derive(Parser)]
#[command(name = "refrs")]
#[command(about = "", version = "0.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Clone { relative_path: String, url: String },
}

fn get_state_file_path() -> PathBuf {
    let mut path = dirs_next::data_local_dir().unwrap_or_else(std::env::temp_dir);
    path.push("refrs");
    path.push("state.yaml");
    path
}

fn load_state() -> Result<AppState> {
    let state_file = get_state_file_path();
    if state_file.exists() {
        let content = fs::read_to_string(&state_file).context("Failed to read state file")?;
        let state: AppState =
            serde_yaml::from_str(&content).context("Failed to parse state file")?;
        Ok(state)
    } else {
        Ok(AppState::default())
    }
}

fn save_state(state: &AppState) -> Result<()> {
    let state_file = get_state_file_path();
    let parent_dir = state_file.parent().unwrap();

    // Ensure the directory exists
    fs::create_dir_all(parent_dir).context("Failed to create state directory")?;

    let content = serde_yaml::to_string(state).context("Failed to serialize state")?;
    fs::write(&state_file, content).context("Failed to write state file")?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load state at the start
    let mut state = load_state()?;

    match &cli.command {
        Commands::Init => {
            println!("Initializing...");
            save_state(&AppState::default())?;
        }

        Commands::Clone { relative_path, url } => {
            let absolute_path = std::env::current_dir()
                .context("Failed to get current working directory")?
                .join(Path::new(relative_path));

            println!("Cloning: {}", url);
            println!("Absolute path: {}", absolute_path.display());

            let status = Command::new("git")
                .args(["clone", url, absolute_path.to_str().unwrap()])
                .status()
                .context("Failed to execute git command")?;

            if !status.success() {
                return Err(anyhow::anyhow!("Git command failed"));
            }

            state.projects.push(Project {
                absolute_path: absolute_path.to_string_lossy().to_string(),
                url: url.clone(),
            });

            save_state(&state)?;
        }
    }

    Ok(())
}
