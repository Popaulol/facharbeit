use std::fmt;

use crate::chess::piece::{Piece, PieceColor, PieceType};
use crate::chess::position::Position;
use colored::*;

fn empty_rank() -> [Option<Piece>; 8] {
    return [None, None, None, None, None, None, None, None];
}

fn pawn_rank(color: PieceColor) -> [Option<Piece>; 8] {
    return [
        Piece::new(color, PieceType::Pawn, false),
        Piece::new(color, PieceType::Pawn, false),
        Piece::new(color, PieceType::Pawn, false),
        Piece::new(color, PieceType::Pawn, false),
        Piece::new(color, PieceType::Pawn, false),
        Piece::new(color, PieceType::Pawn, false),
        Piece::new(color, PieceType::Pawn, false),
        Piece::new(color, PieceType::Pawn, false),
    ];
}

fn piece_rank(color: PieceColor) -> [Option<Piece>; 8] {
    return [
        Piece::new(color, PieceType::Rook, false),
        Piece::new(color, PieceType::Knight, false),
        Piece::new(color, PieceType::Bishop, false),
        Piece::new(color, PieceType::Queen, false),
        Piece::new(color, PieceType::King, false),
        Piece::new(color, PieceType::Bishop, false),
        Piece::new(color, PieceType::Knight, false),
        Piece::new(color, PieceType::Rook, false),
    ];
}

#[derive(Debug, Copy, Clone)]
pub struct Board {
    board: [[Option<Piece>; 8]; 8],
    white_en_passant: [bool; 8],
    black_en_passant: [bool; 8],
    white_castle_king: bool,
    white_castle_queen: bool,
    black_castle_king: bool,
    black_castle_queen: bool,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [
                piece_rank(PieceColor::White),
                pawn_rank(PieceColor::White),
                empty_rank(),
                empty_rank(),
                empty_rank(),
                empty_rank(),
                pawn_rank(PieceColor::Black),
                piece_rank(PieceColor::Black),
            ],
            white_en_passant: [false, false, false, false, false, false, false, false],
            black_en_passant: [false, false, false, false, false, false, false, false],
            white_castle_king: true,
            white_castle_queen: true,
            black_castle_king: true,
            black_castle_queen: true
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut color = true;
        write!(f, "   | A  B  C  D  E  F  G  H |   \n")?;
        write!(f, "---+------------------------+---\n")?;
        for (i, rank) in self.board.iter().enumerate().rev() {
            write!(f, " {} |", i + 1)?;
            for piece in rank {
                let out = if let Some(piece) = piece {
                    format!("{}", piece)
                } else {
                    "   ".to_string()
                };
                if color {
                    write!(f, "{}", out.black().on_white())?;
                } else {
                    write!(f, "{}", out.white().on_black())?;
                }
                color = !color;
            }
            write!(f, "| {} \n", i + 1)?;
            color = !color;
        }

        write!(f, "---+------------------------+---\n")?;
        write!(f, "   | A  B  C  D  E  F  G  H |   \n")
    }
}

impl Board {
    pub(crate) fn at_pos(&self, p: Position) -> Option<Piece> {
        self.board[p.rank()][p.file()]
    }
    pub(crate) fn at(&self, file: usize, rank: usize) -> Option<Piece> {
        self.at_pos(Position::new(file, rank))
    }

    fn set(&mut self, p: Position, piece: Option<Piece>) {
        self.board[p.rank()][p.file()] = piece
    }

    pub(crate) fn r#move(mut self, from: Position, to: Position) -> Self {
        self.set(to, self.at_pos(from));
        self.set(from, None);
        self
    }

    pub fn check_piece_pos(&self, p: Position) -> bool {
        !self.at_pos(p).is_none()
    }

    pub fn check_piece(&self, file: usize, rank: usize) -> bool {
        !self.at(file, rank).is_none()
    }
}
