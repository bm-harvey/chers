use std::error::Error;
use std::fmt;

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

pub mod bits {

    pub fn square_idx_to_mask(square: usize) -> u64 {
        0b1_u64 << square
    }

    pub fn square_mask_to_idx(square: u64) -> usize{
        // assumes exactly one bit is on
        square.trailing_zeros() as usize
    }


    pub fn square_exists_left(square: usize) -> bool {
        square % 8 > 0
    }
    pub fn square_exists_right(square: usize) -> bool {
        square % 8 < 7
    }
    pub fn square_exists_down(square: usize) -> bool {
        square / 8 > 0
    }
    pub fn square_exists_up(square: usize) -> bool {
        square / 8 < 7
    }

    pub fn square_exists_two_left_mask(square_mask: u64) -> bool {
        let row_mask = 0x03_03_03_03_03_03_03_03; // turns on bits in left two columns
        square_mask & row_mask == 0
    }

    pub fn square_exists_two_right_mask(square_mask: u64) -> bool {
        let row_mask = 0xc0_c0_c0_c0_c0_c0_c0_c0; // turns on bits in right two columns
        square_mask & row_mask == 0
    }
    pub fn square_exists_two_down_mask(square_mask: u64) -> bool {
        let row_mask = 0x00_00_00_00_00_00_ff_ff; // turns on bits in botton row
        square_mask & row_mask == 0
    }
    pub fn square_exists_two_up_mask(square_mask: u64) -> bool {
        let row_mask = 0xff_ff_00_00_00_00_00_00; // turns on bits in top row
        square_mask & row_mask == 0
    }

    pub fn square_exists_left_mask(square_mask: u64) -> bool {
        let row_mask = 0x01_01_01_01_01_01_01_01; // turns on bits in left column
        square_mask & row_mask == 0
    }

    pub fn square_exists_right_mask(square_mask: u64) -> bool {
        let row_mask = 0x80_80_80_80_80_80_80_80; // turns on bits in right column
        square_mask & row_mask == 0
    }
    pub fn square_exists_down_mask(square_mask: u64) -> bool {
        let row_mask = 0x00_00_00_00_00_00_00_ff; // turns on bits in botton row
        square_mask & row_mask == 0
    }
    pub fn square_exists_up_mask(square_mask: u64) -> bool {
        let row_mask = 0xff_00_00_00_00_00_00_00; // turns on bits in top row
        square_mask & row_mask == 0
    }

    pub fn dbg_mask(mask: u64) {
        println!();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = 8 * rank + file;

                let square_on = (mask & (0b1_u64 << square)) != 0;

                if square_on {
                    print!("1 ");
                } else {
                    print!("0 ");
                }
            }
            println!();
        }
    }
}
