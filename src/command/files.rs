use std::fs;
use std::path::Path;

use crate::model::ris::{self, ris_entry_to_bibtex_string};
use crate::services::serialization;
use crate::state::AppState;
use crate::util::print_not_initialized;
use anyhow::Result;
use arboard::Clipboard;
use colored::Colorize;

fn print_problematic_line(text: &str, start: usize, end: usize) {
    let lines: Vec<&str> = text.lines().collect();
    let mut char_count = 0;

    for (line_number, line) in lines.iter().enumerate() {
        let line_start = char_count;
        let line_end = char_count + line.len();

        if start >= line_start && start < line_end {
            // Print the problematic line
            println!("Line {}: {}", line_number + 1, line);

            // Calculate the offset of the problem in the line
            let indicator_start = start - line_start;
            let indicator_end = (end - line_start).min(line.len());

            // Print an arrow pointing to the problem
            let mut indicator = String::new();
            indicator.push_str(&" ".repeat(indicator_start));
            indicator.push_str(&"-".repeat(indicator_end - indicator_start));
            indicator.push('^');
            println!("{}{}", " ".repeat(6), indicator); // Align with "Line X: "
            return;
        }

        char_count += line.len() + 1; // Include the newline character
    }

    println!("Unexpected end of bibtex.");
}

pub fn handle_import(state: &AppState, from_clipboard: bool) -> Result<()> {
    if !state.initialized {
        print_not_initialized();
        return Ok(());
    }

    if state.projects.is_empty() {
        println!("{}", "No project selected.".blue().bold());
        return Ok(());
    }

    let text: String;
    if from_clipboard {
        let mut clipboard = Clipboard::new()?;
        text = clipboard.get_text()?;
    } else {
        println!(
            "{}: Currenlty only clipboard is supported. Use: {}",
            "Warning".bold().yellow(),
            "refrs import --clipboard".bold()
        );
        return Ok(());
    }

    match serialization::import(&text, &state.current_project)? {
        serialization::ImportResult::BibtexImported => {}
        serialization::ImportResult::BibtexError { error } => {
            print_problematic_line(&text, error.span.start, error.span.end);
        }
        serialization::ImportResult::RisImported => {}
        serialization::ImportResult::RisError { error } => {
            println!("{}", error);
        }
        serialization::ImportResult::UnrecognizedFormat => {
            println!(
                "Did not recognize text format. Supported formats: {}, {}",
                "BibTex".bold(),
                "RIS".bold()
            );
        }
    }

    Ok(())
}

pub fn handle_export(state: &AppState, file_name: &String) -> Result<()> {
    // Ensure the state is initialized
    if !state.initialized {
        print_not_initialized();
        return Ok(());
    }

    // Check if a project is selected
    if state.projects.is_empty() {
        println!("{}", "No project selected.".blue().bold());
        return Ok(());
    }

    let project_path = &state.current_project;
    let ris_folder = "ris_files";
    let ris_folder_path = Path::new(project_path).join(ris_folder);

    // Ensure the ris_files folder exists
    if !ris_folder_path.exists() {
        println!("{}", "No ris_files folder found.".red().bold());
        return Ok(());
    }

    // Collect all .ris files in the folder
    let mut bibtex_entries = String::new();

    for entry in fs::read_dir(&ris_folder_path)? {
        let entry = entry?;
        let path = entry.path();

        // Process only .ris files
        if let Some(extension) = path.extension() {
            if extension == "ris" {
                // Read the .ris file
                let content = fs::read_to_string(&path)?;

                // Parse the RIS content
                match ris::parse_ris(&content) {
                    Ok(entries) => {
                        for ris_entry in entries {
                            // Generate a unique entry key based on the file name
                            let entry_key = path
                                .file_stem()
                                .and_then(|os_str| os_str.to_str())
                                .unwrap_or("unknown");

                            // Convert RIS entry to BibTeX
                            let bibtex_entry = ris_entry_to_bibtex_string(&ris_entry, entry_key);
                            bibtex_entries.push_str(&bibtex_entry);
                            bibtex_entries.push('\n'); // Add a newline between entries
                        }
                    }
                    Err(err) => {
                        eprintln!("Error parsing RIS file {}: {}", path.display(), err);
                    }
                }
            }
        }
    }

    // Write the concatenated BibTeX entries to the specified file
    let output_path = Path::new(file_name);
    fs::write(output_path, bibtex_entries)?;

    println!("BibTeX entries exported to {}", output_path.display());

    Ok(())
}
