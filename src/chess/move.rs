use crate::chess::board::Board;
use crate::chess::position::Position;
use std::rc::Rc;
use colored::Color;
use crate::chess::piece::{Piece, PieceColor, PieceType};

pub(crate) struct Move {
    nop_move: bool,
    previous_move: Option<Rc<Move>>,
    pub(crate) board: Rc<Board>,
    from: Option<Position>,
    to: Option<Position>,
}

impl Move {
    pub fn start_position() -> Rc<Self> {
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

    pub(crate) fn from_algebraic(previous_move: Rc<Move>, notation: &str) -> Option<Rc<Self>> {
        if notation.len() != 4 {
            None
        } else {
            let from = &notation[..2];
            let to = &notation[2..];
            println!("{}", from);
            println!("{}", to);
            Some(Self::new(previous_move, Position::from_human_readable(from)?, Position::from_human_readable(to)?))
        }
    }

    pub(crate) fn validate(&self, current_player: PieceColor) -> bool {
        if self.previous_move.is_none() {
            return true
        }
        let previous_move = self.previous_move.clone().unwrap();
        if self.nop_move {
            return self.from.is_none() && self.to.is_none()
        }
        let from = if self.from.is_none() {return false} else {
            self.from.unwrap()
        };
        let to = if self.to.is_none() {return false} else {
            self.to.unwrap()
        };

        let piece = previous_move.board.at_pos(from);
        let piece = if piece.is_none() {return false} else {
            piece.unwrap()
        };

        if piece.color != current_player {
            return false
        }

        match piece.kind {
            PieceType::Pawn => {
                match piece.color {
                    PieceColor::White => {
                        if from.file() == to.file() && from.rank() + 1 == to.rank()  && !previous_move.board.check_piece_pos(to) {
                            true
                        }
                        else if from.file() + 1 == to.file() && from.rank() + 1 == to.rank() && previous_move.board.check_piece_pos(to){
                            true
                        }
                            // TODO: En Passant
                        else if from.file() != 0 && from.file() - 1 == to.file() && from.rank() + 1 == to.rank() && previous_move.board.check_piece_pos(to){
                            true
                        }
                        else if from.file() == to.file() && from.rank() == 1 && to.rank() == 3 && !previous_move.board.check_piece_pos(to) && !previous_move.board.check_piece(from.file(), 2) {
                            true
                        }
                        else {
                            false
                        }
                    }
                    PieceColor::Black => {true}
                }
            }
            PieceType::Rook => {false}
            PieceType::Knight => {false}
            PieceType::Bishop => {false}
            PieceType::Queen => {false}
            PieceType::King => {false}
        }
    }
}
