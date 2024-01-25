use crate::chess::board::Board;

mod chess;

fn main() {
    let b = Board::default();
    println!("{}", b);
}
