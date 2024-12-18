use colored::Colorize;

pub fn print_not_initialized() {
    println!(
        "{}{}{}",
        "Warning: ".yellow().bold(),
        "Rustrs not initialized. To initialize, run: ",
        "rustrs init".bold()
    );
}
