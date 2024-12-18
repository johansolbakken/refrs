use anyhow::Result;
use crate::state::{AppState, Project};
use crate::repo;

pub fn handle_clone(state: &mut AppState, relative_path: &str, url: &str) -> Result<()> {
    let absolute_path = repo::clone_repo(relative_path, url)?;
    state.projects.push(Project {
        absolute_path,
        url: url.to_string(),
    });
    Ok(())
}
