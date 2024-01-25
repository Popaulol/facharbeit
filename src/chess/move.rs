use crate::chess::board::Board;
use crate::chess::position::Position;
use std::rc::Rc;

struct Move {
    previous_move: Option<Rc<Move>>,
    board: Rc<Board>,
    from: Position,
    to: Position,
}

impl Move {
    fn new(previous_move: Option<Rc<Move>>) {}
}
