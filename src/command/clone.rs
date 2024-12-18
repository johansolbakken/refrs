use anyhow::Result;
use crate::state::{save_state, AppState};
use crate::state::Project;
use crate::repo;
use crate::util::print_not_initialized;

pub fn handle_clone(state: &mut AppState, relative_path: &str, url: &str) -> Result<()> {
    if !state.initialized {
        print_not_initialized();
        return Ok(());
    }

    let absolute_path = repo::clone_repo(relative_path, url)?;
    state.projects.push(Project {
        absolute_path,
        url: url.to_string(),
    });

    save_state(&state)?;
    Ok(())
}
