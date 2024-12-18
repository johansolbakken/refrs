use anyhow::Result;
use crate::state::{save_state, AppState};

pub fn handle_init() -> Result<()> {
    println!("Initializing...");
    save_state(&AppState::default())?;
    Ok(())
}
