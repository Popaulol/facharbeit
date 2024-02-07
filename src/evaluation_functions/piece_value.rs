use chess::{Board, BoardStatus, Color, Piece};

pub fn color_piece_value(board: Board, color: Color) -> i32 {
    let color_board = board.color_combined(color);

    let queen_count = (board.pieces(Piece::Queen) & color_board).popcnt() as i32;
    let bishop_count = (board.pieces(Piece::Bishop) & color_board).popcnt() as i32;
    let knight_count = (board.pieces(Piece::Knight) & color_board).popcnt() as i32;
    let rock_count = (board.pieces(Piece::Rook) & color_board).popcnt() as i32;
    let pawn_count = (board.pieces(Piece::Pawn) & color_board).popcnt() as i32;

    queen_count * 9 + bishop_count * 3 + knight_count * 3 + rock_count * 5 + pawn_count
}

pub fn piece_value(board: Board) -> i32 {
    match board.status() {
        BoardStatus::Ongoing => {
            color_piece_value(board, board.side_to_move())
                - color_piece_value(board, !board.side_to_move())
        }
        BoardStatus::Stalemate => 0,
        BoardStatus::Checkmate => i32::MAX,
    }
}
