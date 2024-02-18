use chess::{Board, BoardStatus, Color, Piece};
use vampirc_uci::{Serializable, UciInfoAttribute, UciMessage};

pub fn color_piece_value(board: Board, color: Color) -> f32 {
    let color_board = board.color_combined(color);

    let queen_count = (board.pieces(Piece::Queen) & color_board).popcnt() as f32;
    let bishop_count = (board.pieces(Piece::Bishop) & color_board).popcnt() as f32;
    let knight_count = (board.pieces(Piece::Knight) & color_board).popcnt() as f32;
    let rock_count = (board.pieces(Piece::Rook) & color_board).popcnt() as f32;
    let pawn_count = (board.pieces(Piece::Pawn) & color_board).popcnt() as f32;

    let ret = queen_count * 9.0 + bishop_count * 3.0 + knight_count * 3.0 + rock_count * 5.0 + pawn_count;
    //println!("{}", UciMessage::Info(vec![UciInfoAttribute::String(format!("{}", ret))]).serialize());
    ret
}

pub fn piece_value(board: Board) -> f32 {

            color_piece_value(board, board.side_to_move())
                - color_piece_value(board, !board.side_to_move())
}
