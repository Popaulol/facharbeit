use crate::chess::position::Position;
use crate::chess::r#move::Move;
use clap::{Parser, Subcommand};

mod chess;
mod cli;

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
    println!("{:?}", cli);

    let start = Move::nop_move();
    let first_move = Move::new(
        start.clone(),
        Position::from_human_readable("e2").unwrap(),
        Position::from_human_readable("e4").unwrap(),
    );
    let second_move = Move::new(
        first_move.clone(),
        Position::from_human_readable("e7").unwrap(),
        Position::from_human_readable("e5").unwrap(),
    );

    println!("{}", start.board);
    println!("{}", first_move.board);
    println!("{}", second_move.board)
}
