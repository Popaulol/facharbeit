use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    color: PieceColor,
    kind: PieceType,
    has_moved: bool,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", {
            match (self.color, self.kind) {
                (PieceColor::White, PieceType::King) => " ♔ ︎",
                (PieceColor::White, PieceType::Queen) => " ♕ ",
                (PieceColor::White, PieceType::Rook) => " ♖︎ ",
                (PieceColor::White, PieceType::Bishop) => " ♗︎ ",
                (PieceColor::White, PieceType::Knight) => " ♘ ",
                (PieceColor::White, PieceType::Pawn) => " ♙ ",

                (PieceColor::Black, PieceType::King) => " ♚ ︎",
                (PieceColor::Black, PieceType::Queen) => " ♛ ",
                (PieceColor::Black, PieceType::Rook) => " ♜︎ ",
                (PieceColor::Black, PieceType::Bishop) => " ♝ ︎",
                (PieceColor::Black, PieceType::Knight) => " ♞ ",
                (PieceColor::Black, PieceType::Pawn) => " ♟︎ ",
            }
        })
    }
}

impl Piece {
    pub fn new(color: PieceColor, kind: PieceType, has_moved: bool) -> Option<Self> {
        Some(Self {
            color,
            kind,
            has_moved,
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
