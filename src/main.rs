use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

mod command;
mod repo;
mod state;
mod util;

#[derive(Parser)]
#[command(name = "refrs")]
#[command(about = "", version = "0.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        force: bool
    },
    Clone { relative_path: String, url: String },
    Show,
    #[command(subcommand)]
    Workspace(WorkspaceSubcommands),
    Update,
}

#[derive(Subcommand)]
enum WorkspaceSubcommands {
    Set,
    Get,
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    let mut state = state::load_state()?;

    match &cli.command {
        Commands::Init { force } => command::init::handle_init(&mut state, *force)?,
        Commands::Clone { relative_path, url } => command::clone::handle_clone(&mut state, relative_path, url)?,
        Commands::Show => command::show::handle_show(&state),
        Commands::Workspace(subcommand) => match subcommand {
            WorkspaceSubcommands::Set => command::workspace::handle_set(&mut state)?,
            WorkspaceSubcommands::Get => command::workspace::handle_get(&state),
        },
        Commands::Update => command::update::handle_update(&state)?,
    }

    Ok(())
}
