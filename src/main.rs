use crate::chess::position::Position;
use crate::chess::r#move::Move;

mod chess;

fn main() {
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
