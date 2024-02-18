use chess::{Board};

pub fn piece_count(board: Board) -> f32 {

            let current_side = board.color_combined(board.side_to_move()).popcnt() as f32;
            let other_side = board.color_combined(!board.side_to_move()).popcnt() as f32;
            /*if (current_side - other_side) != 0 {
                println!("{} {:?}", current_side - other_side, board.side_to_move());
            }*/
            current_side - other_side

}
