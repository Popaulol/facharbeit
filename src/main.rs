use clap::{Parser, Subcommand};

mod cli;
mod evaluation_functions;
mod minimax;
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
    /// Run the engine in uci mode, this is the default behavior when no subcommand is supplied
    UCI,
}

fn main() {
    let cli = CLI::parse();

    let command = cli.command.unwrap_or(Commands::UCI);

    match command {
        Commands::CLI => cli::cli_main(),
        Commands::UCI => uci::uci_main(),
    }
}
