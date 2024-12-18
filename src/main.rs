use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

mod command;
mod repo;
mod state;

#[derive(Parser)]
#[command(name = "refrs")]
#[command(about = "", version = "0.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Clone { relative_path: String, url: String },
    Show,
    #[command(subcommand)]
    Workspace(WorkspaceSubcommands),
}

#[derive(Subcommand)]
enum WorkspaceSubcommands {
    Set,
    Get,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut state = state::load_state()?;

    match &cli.command {
        Commands::Init => command::init::handle_init()?,
        Commands::Clone { relative_path, url } => command::clone::handle_clone(&mut state, relative_path, url)?,
        Commands::Show => command::show::handle_show(&state),
        Commands::Workspace(subcommand) => match subcommand {
            WorkspaceSubcommands::Set => command::workspace::handle_set(&mut state)?,
            WorkspaceSubcommands::Get => command::workspace::handle_get(&state),
        },
    }

    Ok(())
}
