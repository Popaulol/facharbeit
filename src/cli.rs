use std::io;
use crate::chess;
use crate::chess::position::Position;
use crate::chess::r#move::Move;

pub(crate) fn cli_main() {
    let start = Move::start_position();
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

pub(crate) fn two_players() -> io::Result<()> {
    let mut previous_move = Move::start_position();
    let mut current_player = chess::piece::PieceColor::White;
    println!("{}", previous_move.board);
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        match Move::from_algebraic(previous_move.clone(), buffer.trim()) {
            None => {
                println!("Unable to Parse the move");
            }
            Some(m) => {
                if !m.validate(current_player) {
                    println!("Invalid Move!");
                } else {
                    current_player = current_player.oposite();
                    previous_move = m;
                    println!("{}", previous_move.board);
                    println!("It's {:?}'s turn!", current_player);
                }
            }
        }
    }

}