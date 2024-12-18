use crate::state::{save_state, AppState};
use anyhow::Result;
use colored::*;

pub fn handle_init(state: &mut AppState, force: bool) -> Result<()> {
    if state.initialized && !force {
        println!(
            "{}",
            "Refrs already initialized. Use option -f to reinitialize"
                .yellow()
                .bold()
        );
        return Ok(());
    }

    println!("{}", "Initializing...".blue().bold());
    *state = AppState::default();
    state.initialized = true;
    save_state(state)?;

    Ok(())
}
