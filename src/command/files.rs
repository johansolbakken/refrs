use std::fs;
use std::path::Path;

use crate::model::ris::{self, ris_entry_to_bibtex_string, RisEntry};
use crate::repo;
use crate::state::AppState;
use crate::util::print_not_initialized;
use anyhow::Result;
use arboard::Clipboard;
use biblatex::{Bibliography, ParseError};
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

    // First we try to parse BibTex
    match Bibliography::parse(&text) {
        Ok(bibliography) => {
            if !bibliography.is_empty() {
                for entry in bibliography.iter() {
                    add_entry(state, &ris::RisEntry::from(entry))?;
                }
                return Ok(());
            }
        }
        Err(ParseError { span, .. }) => {
            println!(
                "{}Recognized BibTeX, but content is broken.",
                "Error: ".red().bold()
            );
            print_problematic_line(&text, span.start, span.end);
            return Ok(());
        }
    }

    // Did not recognize bibtex, try RIS
    match ris::parse_ris(&text) {
        Ok(entries) => {
            if !entries.is_empty() {
                for entry in entries.iter() {
                    add_entry(state, entry)?;
                }
                return Ok(());
            }
        }
        Err(err) => {
            // Gracefully return if error
            log::error!(
                "{}Tried to parse RIS, but got: {}.",
                "Error: ".red().bold(),
                err
            );
        }
    }

    println!(
        "Did not recognize text format. Supported formats: {}, {}",
        "BibTex".bold(),
        "RIS".bold()
    );

    Ok(())
}

fn add_entry(state: &AppState, entry: &RisEntry) -> Result<()> {
    let project_path = state.current_project.clone();
    let ris_folder = "ris_files";
    let ris_folder_path = Path::new(&project_path).join(ris_folder);

    if let Err(e) = fs::create_dir_all(&ris_folder_path) {
        eprintln!(
            "Error creating directory {}: {}",
            ris_folder_path.display(),
            e
        );
        return Ok(());
    }

    let academic_stopwords = [
        "a", "an", "and", "the", "of", "in", "on", "for", "with", "to", "from", "by", "about",
        "as", "at", "into", "through", "between", "within", "without", "or", "nor", "but", "yet",
        "so", "because", "although", "since", "while", "when", "where", "that", "which", "what",
        "who", "whose", "whom", "how", "why", "it", "its", "this", "these", "those", "there",
        "here", "such", "more", "less", "many", "much", "any", "every", "each", "other", "some",
        "few", "all", "both", "either", "neither", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine", "ten", "up", "down", "out", "over", "under", "above", "below",
        "new", "current", "recent", "future", "analysis", "study", "research", "results", "review",
        "overview",
    ];

    let title = first_non_stopword(
        match entry.get_field("TI") {
            Some(title) => title.trim(),
            None => "notitle",
        },
        &academic_stopwords,
    )
    .unwrap_or("notitle".to_string())
    .to_lowercase();

    let author = match entry.get_field("AU") {
        Some(author) => {
            // Split the author's name by comma and take the first part (last name)
            author.split(',').next().unwrap_or("noauthor").trim()
        }
        None => "noauthor",
    }
    .to_lowercase();

    let year = match entry.get_field("PY") {
        Some(date) => date.trim(),
        None => "nodate",
    };

    // Sanitize title and author to avoid invalid file characters
    let sanitized_title = title.replace(|c: char| !c.is_alphanumeric(), "_");
    let sanitized_author = author.replace(|c: char| !c.is_alphanumeric(), "_");

    let mut file_name = format!("{}_{}_{}.ris", sanitized_author, sanitized_title, year);
    let mut file_path = ris_folder_path.join(&file_name);

    // Check if file exists and append (1), (2), ... if necessary
    let mut counter = 1;
    while file_path.exists() {
        file_name = format!(
            "{}_{}_{}_{}.ris",
            sanitized_author, sanitized_title, year, counter
        );
        file_path = ris_folder_path.join(&file_name);
        counter += 1;
    }

    // Write the RIS entry to the file
    if let Err(e) = fs::write(&file_path, entry.to_string()) {
        eprintln!("Error writing to file {}: {}", file_path.display(), e);
    } else {
        println!("Entry saved to {}", file_path.display());
    }

    let commit_message = format!("Added {}", file_name);

    repo::add_all(&project_path)?;
    repo::commit(&project_path, &commit_message)?;

    Ok(())
}

fn first_non_stopword(input: &str, stopwords: &[&str]) -> Option<String> {
    // Convert the stopwords array into a HashSet for faster lookup
    let stopwords_set: std::collections::HashSet<_> = stopwords.iter().copied().collect();

    // Split the input into words, filter out stopwords, and return the first non-stopword
    input
        .split_whitespace() // Split the string into words
        .filter(|word| !stopwords_set.contains(*word)) // Remove stopwords
        .next() // Get the first non-stopword
        .map(|word| word.to_string()) // Convert it to a String
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
                        eprintln!(
                            "Error parsing RIS file {}: {}",
                            path.display(),
                            err
                        );
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
