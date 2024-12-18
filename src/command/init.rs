use anyhow::Result;
use crate::state::{save_state, AppState};

pub fn handle_init(state: &mut AppState, force: bool) -> Result<()> {
    if state.initialized && !force {
        println!("Refrs already initialized.");
        return Ok(());
    }

    println!("Initializing...");
    *state = AppState::default();
    state.initialized = true;
    save_state(state)?;

    Ok(())
}
