use crate::chess::board::Board;
use crate::chess::position::Position;
use std::rc::Rc;

pub(crate) struct Move {
    nop_move: bool,
    previous_move: Option<Rc<Move>>,
    pub(crate) board: Rc<Board>,
    from: Option<Position>,
    to: Option<Position>,
}

impl Move {
    pub fn nop_move() -> Rc<Self> {
        Self {
            nop_move: true,
            previous_move: None,
            board: Rc::new(Default::default()),
            from: None,
            to: None,
        }
        .into()
    }
    pub(crate) fn new(previous_move: Rc<Move>, from: Position, to: Position) -> Rc<Self> {
        let mut board = previous_move.clone().board.clone();

        let board = board.r#move(from, to).into();

        Rc::new(Self {
            nop_move: false,
            previous_move: Some(previous_move),
            board,
            from: Some(from),
            to: Some(to),
        })
    }

    fn validate(&self) -> bool {
        // if let
        true
    }
}
