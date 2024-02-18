#![feature(test)]

use std::str::FromStr;
use chess::{Board, ChessMove, Game};
use clap::{Parser, Subcommand};
use crate::evaluation_functions::attacks::attacks;
use crate::evaluation_functions::pawn_pos::pawn_pos;
use crate::evaluation_functions::piece_count::piece_count;
use crate::evaluation_functions::piece_value::piece_value;
use crate::minimax::{minimax_ab, minimax_td, minimax_traditional};

mod cli;
mod evaluation_functions;
mod minimax;
mod uci;
#[cfg(test)]
mod tests;

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
    /// Run For testing
    Test
}

fn main() {
    let cli = CLI::parse();

    let command = cli.command.unwrap_or(Commands::UCI);

    match command {
        Commands::CLI => cli::cli_main(),
        Commands::UCI => uci::uci_main(),
        Commands::Test => {
            let board = Board::from_str("r1bqk2r/2ppbppp/p1n2n2/1p2p3/4P3/1B3N2/PPPP1PPP/RNBQR1K1 b kq - 0 1").unwrap();
            let swapped = board.null_move().unwrap();
            println!("{}", pawn_pos(Board::default()));
            println!("{}", pawn_pos(Board::default().null_move().unwrap()));
            println!("{}", pawn_pos(board));
            println!("{}", pawn_pos(swapped));
        }
    }
}
