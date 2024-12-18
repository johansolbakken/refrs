use crate::state::AppState;

pub fn handle_show(state: &AppState) {
    if state.projects.is_empty() {
        println!("No projects found.");
        return;
    }

    println!("{:<30} | {:<50}", "Absolute Path", "URL");
    println!("{:-<80}", "-");

    for project in &state.projects {
        println!("{:<30} | {:<50}", project.absolute_path, project.url);
    }
}
