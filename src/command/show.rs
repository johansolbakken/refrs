use colored::Colorize;

use crate::{state::AppState, util::print_not_initialized};

pub fn handle_show(state: &AppState) {
    if !state.initialized {
        print_not_initialized();
        return;
    }

    if state.projects.is_empty() {
        println!("{}", "No projects found.".blue().bold());
        return;
    }

    // Header with styled text
    println!("{}", "# Projects".green().bold());
    println!("{:<30} | {:<50}", "Absolute Path".underline(), "URL".underline());
    println!("{:-<80}", "-");

    // Iterate through projects and display them
    for project in &state.projects {
        println!("{:<30} | {:<50}", project.absolute_path, project.url);
    }
}
