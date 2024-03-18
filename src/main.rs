#![feature(test)]
#![feature(panic_update_hook)]

use std::panic;
use std::process::Command;
use chess::{Board, ChessMove, Game, Square};
use clap::{Parser, Subcommand};
use std::str::FromStr;
use crate::evaluation_functions::attacks::{attacks, attacks_diff};

use crate::evaluation_functions::piece_tables::piece_tables;
use crate::evaluation_functions::piece_value::piece_value;
use crate::minimax::negamax;

mod cli;
mod evaluation_functions;
mod minimax;
#[cfg(test)]
mod tests;
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
    /// Run For testing
    Test,
}

pub fn message(s: &str) {
    Command::new("notify-send")
        .arg(std::process::id().to_string())
        .arg(s)
        .spawn().unwrap().wait().unwrap();

}

fn main() {
    panic::update_hook(move |prev, info| {
        message(info.to_string().as_str());
        prev(info);
    });
    
    let cli = CLI::parse();

    let command = cli.command.unwrap_or(Commands::UCI);

    match command {
        Commands::CLI => cli::cli_main(),
        Commands::UCI => uci::uci_main(),
        Commands::Test => {
            // uci::uci_main();
            let board = Board::from_str(
                "r1bqk2r/2ppbppp/p1n2n2/1p2p3/4P3/1B3N2/PPPP1PPP/RNBQR1K1 b kq - 0 1",
            )
            .unwrap();
            let swapped = board.null_move().unwrap();
            println!("{}", piece_tables(Board::default()));
            println!("{}", piece_tables(Board::default().null_move().unwrap()));
            println!("{}", piece_tables(board));
            println!("{}", piece_tables(swapped));

            println!("{}", negamax(Game::new(), 4, attacks));
            println!("{}", negamax(Game::new(), 4, attacks_diff));


            let b1c3 = ChessMove::new(Square::B1, Square::C3, None);
            let c3b1 = ChessMove::new(Square::C3, Square::B1, None);

            let b8c6 = ChessMove::new(Square::B8, Square::C6, None);
            let c6b8 = ChessMove::new(Square::C6, Square::B8, None);

            let mut game = Game::from_str("rnbqkbnr/pppp3p/6p1/4pp1Q/5P2/4P3/PPPP2PP/RNB1KBNR w KQkq - 0 4").unwrap();
            
            let eval_fn = attacks;
            
            for i in 0..4 {
                println!("0 {} {}", i, negamax(game.clone(), i, eval_fn));
            }
            
            game.make_move(b1c3);
            game.make_move(b8c6);
            game.make_move(c3b1);
            game.make_move(c6b8);

            for i in 0..4 {
                println!("1 {} {}", i, negamax(game.clone(), i, eval_fn));
            }

            game.make_move(b1c3);
            game.make_move(b8c6);
            game.make_move(c3b1);
            game.make_move(c6b8);

            for i in 0..4 {
                println!("2 {} {}", i, negamax(game.clone(), i, eval_fn));
            }

            game.declare_draw();
            
            println!("{}", piece_tables(Board::default()))
        }
    }
}
