use chess::Board;
use crate::evaluation_functions::attacks::attacks_diff;
use crate::evaluation_functions::piece_tables::piece_tables;
use crate::evaluation_functions::piece_value::piece_value;

pub fn combined(board: Board) -> f32 {
    0.5 * attacks_diff(board) +
    1.0 * piece_tables(board) + 
    10.0 * piece_value(board)
}