use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

mod repo;
mod command;
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut state = state::load_state()?;

    match &cli.command {
        Commands::Init => {
            command::init::handle_init()?;
        }
        Commands::Clone { relative_path, url } => {
            command::clone::handle_clone(&mut state, relative_path, url)?;
            state::save_state(&state)?;
        }
        Commands::Show => {
            command::show::handle_show(&state);
        }
    }

    Ok(())
}
