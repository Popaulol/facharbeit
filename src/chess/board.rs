use std::fmt;

use crate::chess::piece::{Piece, PieceColor, PieceType};

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
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [
                piece_rank(PieceColor::Black),
                pawn_rank(PieceColor::Black),
                empty_rank(),
                empty_rank(),
                empty_rank(),
                empty_rank(),
                pawn_rank(PieceColor::White),
                piece_rank(PieceColor::White),
            ],
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "   | A  B  C  D  E  F  G  H |   \n")?;
        write!(f, "---+------------------------+---\n")?;
        for (i, rank) in self.board.iter().enumerate() {
            write!(f, " {} |", 8 - i)?;
            for piece in rank {
                if let Some(piece) = piece {
                    write!(f, "{}", piece)?
                } else {
                    write!(f, "   ")?
                }
            }
            write!(f, "| {} \n", 8 - i)?
        }
        write!(f, "---+------------------------+---\n")?;
        write!(f, "   | A  B  C  D  E  F  G  H |   \n")
    }
}
