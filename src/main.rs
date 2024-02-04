use crate::chess::position::Position;
use crate::chess::r#move::Move;
use clap::{Parser, Subcommand};

mod chess;
mod cli;
mod uci;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Play against the engine on the Command Line
    CLI,
    /// Play against another Person on the Command Lina
    TwoPlayer,
    /// Run the engine in uci mode, this is the default behavior when no subcommand is supplied
    UCI,
}

fn main() {
    let cli = CLI::parse();

    let command = cli.command.unwrap_or(Commands::UCI);

    match command {
        Commands::CLI => cli::cli_main(),
        Commands::TwoPlayer => cli::two_players().unwrap(),
        Commands::UCI => uci::uci_main(),
    }
}
