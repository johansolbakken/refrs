use std::fs;

use anyhow::anyhow;
use anyhow::Result;
use colored::Colorize;

use crate::model::ris::{parse_ris, RisEntry};

pub fn print_not_initialized() {
    println!(
        "{}{}{}",
        "Warning: ".yellow().bold(),
        "Rustrs not initialized. To initialize, run: ",
        "rustrs init".bold()
    );
}

pub fn read_ris_files_from_dir(dir: &str) -> Result<Vec<RisEntry>> {
    let mut entries = Vec::new();

    // Read the directory
    let paths = fs::read_dir(dir).map_err(|e| anyhow!("Failed to read directory: {}", e))?;

    for path in paths {
        let path = path?.path();

        // Check if the file has a `.ris` extension
        if path.extension().map(|ext| ext == "ris").unwrap_or(false) {
            // Read the file content
            let content = fs::read_to_string(&path)
                .map_err(|e| anyhow!("Failed to read file {:?}: {}", path, e))?;

            // Parse the RIS content
            let file_entries = parse_ris(&content).map_err(|e| {
                anyhow!(
                    "Failed to parse RIS content in file {:?}: {}",
                    path.file_name().unwrap_or_default(),
                    e
                )
            })?;

            // Append parsed entries to the result vector
            entries.extend(file_entries);
        }
    }

    Ok(entries)
}
