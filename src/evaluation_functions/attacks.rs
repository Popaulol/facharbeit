use chess::{Board, MoveGen, Piece};

pub fn attacks(board: Board) -> f32 {
    let iterator = MoveGen::new_legal(&board);
    let moves = iterator.map(|m| {
        if let Some(piece) = board.piece_on(m.get_dest()) {
            match piece {
                Piece::Pawn => 1.0,
                Piece::Knight => 3.0,
                Piece::Bishop => 3.0,
                Piece::Rook => 5.0,
                Piece::Queen => 9.0,
                Piece::King => 100.0,
            }
        } else {
            0.0
        }
    });
    moves.sum()
}

pub fn attacks_diff(board: Board) -> f32 {
    let enemy_attacks = match board.null_move() {
        None => f32::INFINITY,
        Some(enemy_board) => attacks(enemy_board),
    };
    attacks(board) - enemy_attacks
}
