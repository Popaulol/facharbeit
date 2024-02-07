use chess::{Board, BoardStatus};

pub fn piece_count(board: Board) -> i32 {
    match board.status() {
        BoardStatus::Ongoing => {
            board.color_combined(board.side_to_move()).popcnt() as i32
                - !board.color_combined(board.side_to_move()).popcnt() as i32
        }
        BoardStatus::Stalemate => 0,
        BoardStatus::Checkmate => i32::MAX,
    }
}
