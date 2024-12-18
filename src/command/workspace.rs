use crate::state::{save_state, AppState};
use anyhow::Result;

pub fn handle_set(state: &mut AppState) ->  Result<()> {
    if state.projects.is_empty() {
        println!("No projects available to select.");
        return Ok(());
    }

    let selected_index = dialoguer::Select::new()
        .with_prompt("Select a workspace to set as current")
        .items(
            &state
                .projects
                .iter()
                .map(|p| &p.absolute_path)
                .collect::<Vec<_>>(),
        )
        .default(0)
        .interact()?;

    state.current_project = state.projects[selected_index].absolute_path.clone();
    save_state(&state)?;
    println!("Current workspace set to: {}", state.current_project);

    Ok(())
}

pub fn handle_get(state: &AppState) {
    if state.current_project.is_empty() {
        println!("No current project is set.");
    } else {
        println!("Current project: {}", state.current_project);
    }
}
