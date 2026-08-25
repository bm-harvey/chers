use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ChersError {
    MoveParseError,
    IllegalMoveError,
}

impl fmt::Display for ChersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MoveParseError => write!(f, "An invalid move format was submitted."),
            Self::IllegalMoveError => write!(f, "An illegal move was attempted."),
        }
    }
}

impl Error for ChersError {}

#[derive(Debug)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

