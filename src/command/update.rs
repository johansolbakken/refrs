use std::path::Path;

use anyhow::Result;
use colored::Colorize;
use crate::state::AppState;
use crate::repo;
use crate::util::print_not_initialized;

pub fn handle_update(state: &AppState) -> Result<()> {
    if !state.initialized {
        print_not_initialized();
        return Ok(());
    }

    if state.current_project.is_empty() {
        println!("{}", "No project selected.".blue().bold());
        println!("To select a project use: {}", "refrs workspace set".bold());
        return Ok(());
    }

    if !Path::new(state.current_project.as_str()).exists() {
        println!("{}{}{}{}",
                 "Error: ".bold().red(),
                 "Failed because path '",
                 state.current_project.as_str().underline().bold(),
                 "' is not existing."
        );
        return Ok(());
    }

    repo::pull_rebase(&state.current_project)?;
    repo::push(&state.current_project)?;

    Ok(())
}
