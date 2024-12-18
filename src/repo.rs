use anyhow::{Context, Result};
use colored::*;
use std::process::Command;
use std::path::Path;

/// Executes a Git command with the provided arguments.
fn execute_git_command<P: AsRef<Path>>(repo_path: P, args: &[&str]) -> Result<()> {
    let status = Command::new("git")
        .current_dir(repo_path.as_ref())
        .args(args)
        .status()
        .context("Failed to execute git command")?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "{}: Git command failed with exit status {}",
            "Error".red().bold(),
            status
        ));
    }

    Ok(())
}

/// Clones a Git repository to the specified path.
pub fn clone_repo(relative_path: &str, url: &str) -> Result<String> {
    let absolute_path = std::env::current_dir()
        .context("Failed to get current working directory")?
        .join(Path::new(relative_path));

    println!(
        "{} {}",
        "Cloning:".green().bold(),
        url.underline().bold()
    );
    println!(
        "{} {}",
        "Absolute path:".cyan(),
        absolute_path.display().to_string().underline().bold()
    );

    execute_git_command(
        std::env::current_dir()?,
        &["clone", url, absolute_path.to_str().unwrap()],
    )?;

    println!("{}", "Repository cloned successfully!".green().bold());

    Ok(absolute_path.to_string_lossy().to_string())
}

/// Performs a `git pull --rebase` in the specified repository.
pub fn pull_rebase(repo_path: &str) -> Result<()> {
    println!(
        "{} {}",
        "Pulling with rebase in:".yellow().bold(),
        repo_path.underline().bold()
    );

    execute_git_command(repo_path, &["pull", "--rebase"])?;

    println!("{}", "Rebase completed successfully!".green().bold());
    Ok(())
}

/// Pushes changes to the remote repository.
pub fn push(repo_path: &str) -> Result<()> {
    println!(
        "{} {}",
        "Pushing changes in:".yellow().bold(),
        repo_path.underline().bold()
    );

    execute_git_command(repo_path, &["push"])?;

    println!("{}", "Push completed successfully!".green().bold());
    Ok(())
}

/// Stages all changes (adds all files) in the specified repository.
pub fn add_all(repo_path: &str) -> Result<()> {
    println!(
        "{} {}",
        "Staging all changes in:".yellow().bold(),
        repo_path.underline().bold()
    );

    execute_git_command(repo_path, &["add", "--all"])?;

    println!("{}", "All changes staged successfully!".green().bold());
    Ok(())
}

/// Commits staged changes with the provided commit message.
pub fn commit(repo_path: &str, message: &str) -> Result<()> {
    println!(
        "{} \"{}\" {}",
        "Committing changes with message:".yellow().bold(),
        message.cyan().italic(),
        repo_path.underline().bold()
    );

    execute_git_command(repo_path, &["commit", "-m", message])?;

    println!("{}", "Commit completed successfully!".green().bold());
    Ok(())
}
