use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub absolute_path: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppState {
    pub projects: Vec<Project>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
        }
    }
}

fn get_state_file_path() -> PathBuf {
    let mut path = dirs_next::data_local_dir().unwrap_or_else(std::env::temp_dir);
    path.push("refrs");
    path.push("state.yaml");
    path
}

pub fn load_state() -> Result<AppState> {
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

pub fn save_state(state: &AppState) -> Result<()> {
    let state_file = get_state_file_path();
    let parent_dir = state_file.parent().unwrap();

    fs::create_dir_all(parent_dir).context("Failed to create state directory")?;

    let content = serde_yaml::to_string(state).context("Failed to serialize state")?;
    fs::write(&state_file, content).context("Failed to write state file")?;
    Ok(())
}
