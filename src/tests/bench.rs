#[allow(soft_unstable)]
extern crate test;
use test::{Bencher, black_box};
use chess::Board;
use crate::evaluation_functions::piece_count::piece_count;
use crate::evaluation_functions::piece_value::piece_value;
use crate::minimax::{minimax_ab, minimax_alpha_beta, minimax_td, minimax_traditional};

#[bench]
fn bench_minimax_traditional(b: &mut Bencher) {
    let board = Board::default();

    b.iter(|| {for i in 0..6 {
            minimax_td(board, i, piece_value);}
    })
}
#[bench]
fn bench_minimax_alpha_beta(b: &mut Bencher) {
    let board = Board::default();
    b.iter(|| {
        for i in 0..6 {
            minimax_ab(board, i, piece_value);
    }})
}
