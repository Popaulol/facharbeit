use chess::{Board, Color, EMPTY, Piece, Square};
const PAWN_PREFERENCES: [f32; 64] = [
    f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY,
    10.0, 10.0, 10.0, 0.0, 10.0, 10.0, 10.0, 10.0,
    0.0, 0.0, 0.0, 5.0, 5.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 10.0, 9.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 12.0, 11.0, 11.0, 12.0, 0.0, 0.0,
    0.0, 14.0, 12.0, 12.0, 12.0, 13.0, 14.0, 0.0,
    16.0, 15.0, 14.0, 13.0, 13.0, 14.0, 15.0, 16.0,
    f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY,
];

pub fn pawn_pos(board: Board) -> f32 {
    let pawns = board.pieces(Piece::Pawn) & board.color_combined(board.side_to_move());
    let pawns = if board.side_to_move() == Color::White {pawns} else {pawns.reverse_colors()};
    println!("{}", pawns);
    pawns.map(|square: Square| {
        PAWN_PREFERENCES[square.to_index()]
    }).sum()
}
