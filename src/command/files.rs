use crate::model::ris::{self, RisEntry};
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
                    add_entry(&ris::RisEntry::from(entry));
                    println!("{:?}", entry);
                }
                return Ok(());
            }
        }
        Err(ParseError { span, .. }) => {
            println!("{}Recognized BibTeX, but content is broken.", "Error: ".red().bold());
            print_problematic_line(&text, span.start, span.end);
            return Ok(());
        }
    }

    // Did not recognize bibtex, try RIS
    match ris::parse_ris(&text) {
        Ok(entries) => {
            for entry in entries.iter() {
                add_entry(entry);
            }
        }
        Err(err) => {
            println!("{}Tried to parse RIS, but got: {}.", "Error: ".red().bold(), err);
            return Ok(());
        }
    }

    println!(
        "Did not recognize text format. Supported formats: {}, {}",
        "BibTex".bold(),
        "RIS".bold()
    );

    Ok(())
}

fn add_entry(entry: &RisEntry) {

}
