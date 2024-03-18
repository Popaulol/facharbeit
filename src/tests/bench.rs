#[allow(soft_unstable)]
extern crate test;
use chess::{Board, Game};
use test::Bencher;

use crate::evaluation_functions::piece_value::piece_value;
use crate::minimax::{minimax_ab, minimax_td, negamax};

#[bench]
fn bench_minimax_traditional(b: &mut Bencher) {
    let board = Board::default();

    b.iter(|| {
        for i in 0..6 {
            minimax_td(board, i, piece_value);
        }
    })
}
#[bench]
fn bench_minimax_alpha_beta(b: &mut Bencher) {
    let board = Board::default();
    b.iter(|| {
        for i in 0..6 {
            minimax_ab(board, i, piece_value);
        }
    })
}
#[bench]
fn bench_negamax(b: &mut Bencher) {
    let board = Game::new();
    b.iter(|| {
        for i in 0..6 {
            negamax(board.clone(), i, piece_value);
        }
    })
}
#[bench]
fn bench_negamax_d6(b: &mut Bencher) {
    let board = Game::new();
    b.iter(|| {
        negamax(board.clone(), 6, piece_value);
    })
}
