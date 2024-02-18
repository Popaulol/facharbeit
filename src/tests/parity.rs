use crate::evaluation_functions::piece_value::piece_value;
use crate::minimax::{minimax_ab, minimax_td};
use chess::Board;
use std::str::FromStr;

#[test]
fn parity_default_board() {
    let board = Board::default();

    for i in 0..6 {
        assert_eq!(
            minimax_td(board, i, piece_value),
            minimax_ab(board, i, piece_value)
        );
    }
}

#[test]
fn parity_custom_board_1() {
    let board =
        Board::from_str("r1bqk2r/2ppbppp/p1n2n2/1p2p3/4P3/1B3N2/PPPP1PPP/RNBQR1K1 b kq - 0 1")
            .unwrap();

    for i in 0..6 {
        assert_eq!(
            minimax_td(board, i, piece_value),
            minimax_ab(board, i, piece_value)
        );
    }
}
#[test]
fn symmetry_default_board() {
    let board = Board::default();
    let swapped = board.null_move().unwrap();

    for i in 0..6 {
        assert_eq!(
            minimax_ab(board, i, piece_value),
            -minimax_ab(swapped, i, piece_value)
        );
    }
}

#[test]
fn symmetry_custom_board_1() {
    let board =
        Board::from_str("r1bqk2r/2ppbppp/p1n2n2/1p2p3/4P3/1B3N2/PPPP1PPP/RNBQR1K1 b kq - 0 1")
            .unwrap();
    let swapped = board.null_move().unwrap();

    for i in 0..6 {
        let first = minimax_ab(board, i, piece_value);
        let second = -minimax_ab(swapped, i, piece_value);
        assert_eq!(first, second);
    }
}
