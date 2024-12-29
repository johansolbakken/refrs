use std::fs;
use std::path::Path;

use crate::model::ris::{self, RisEntry};
use crate::repo;
use anyhow::Result;
use biblatex::{Bibliography, ParseError};

pub enum ImportResult {
    BibtexImported,
    BibtexError{error: ParseError},
    RisImported,
    RisError{error: anyhow::Error},
    UnrecognizedFormat,
}


pub fn import(text: &String, project_path: &String) -> Result<ImportResult> {
    fs::create_dir_all(project_path)?;

    println!("{project_path}");

    match Bibliography::parse(&text) {
        Ok(bibliography) => {
            if !bibliography.is_empty() {
                for entry in bibliography.iter() {
                    add_entry(&ris::RisEntry::from(entry), project_path)?;
                }
                return Ok(ImportResult::BibtexImported);
            }
        }
        Err(error) => {
            return Ok(ImportResult::BibtexError { error });
        }
    }

    // Did not recognize bibtex, try RIS
    match ris::parse_ris(&text) {
        Ok(entries) => {
            if !entries.is_empty() {
                for entry in entries.iter() {
                    add_entry(entry, project_path)?;
                }
                return Ok(ImportResult::RisImported);
            }
        }
        Err(error) => {
            return Ok(ImportResult::RisError{error})
        }
    }

    Ok(ImportResult::UnrecognizedFormat)
}

pub fn add_entry(entry: &RisEntry, project_path: &String) -> Result<()> {
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
    fs::write(&file_path, entry.to_string())?;

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
