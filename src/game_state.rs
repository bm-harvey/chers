use std::{char, fmt};

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

pub enum PieceColor {
    White,
    Black,
}

pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    pub fn render_char(&self, color: &PieceColor, render_type: &RenderType) -> char {
        let (white_symbols, black_symbols) = match render_type {
            RenderType::ASCII => {
                return match color {
                    PieceColor::White => match self {
                        PieceType::Pawn => 'P',
                        PieceType::Rook => 'R',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Queen => 'Q',
                        PieceType::King => 'K',
                    },
                    PieceColor::Black => match self {
                        PieceType::Pawn => 'p',
                        PieceType::Rook => 'r',
                        PieceType::Knight => 'n',
                        PieceType::Bishop => 'b',
                        PieceType::Queen => 'q',
                        PieceType::King => 'k',
                    },
                };
            }
            RenderType::Normal => (
                [
                    '\u{2659}', '\u{2656}', '\u{2658}', '\u{2657}', '\u{2655}', '\u{2654}',
                ],
                [
                    '\u{265F}', '\u{265C}', '\u{265E}', '\u{265D}', '\u{265B}', '\u{265A}',
                ],
            ),
            RenderType::Inverted => (
                [
                    '\u{265F}', '\u{265C}', '\u{265E}', '\u{265D}', '\u{265B}', '\u{265A}',
                ],
                [
                    '\u{2659}', '\u{2656}', '\u{2658}', '\u{2657}', '\u{2655}', '\u{2654}',
                ],
            ),
        };
        let idx = match self {
            PieceType::Pawn => 0,
            PieceType::Rook => 1,
            PieceType::Knight => 2,
            PieceType::Bishop => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        };
        match color {
            PieceColor::White => white_symbols[idx],
            PieceColor::Black => black_symbols[idx],
        }
    }
}

#[derive(Default)]
pub enum RenderType {
    #[default]
    ASCII,
    Normal,
    Inverted, // for dark terminals
}

pub struct GameViewer {
    render_type: RenderType,
}

impl<'a> GameViewer {
    pub fn new() -> Self {
        Self {
            render_type: RenderType::ASCII,
        }
    }
    pub fn with_render_type(mut self, render_type: RenderType) -> Self {
        self.render_type = render_type;
        self
    }
    pub fn print(&self, game: &Game) -> () {
        let mut out_chars = ['*'; 64];

        for square in 0..64 {
            out_chars[square] = match game.board_state.piece_in_square(square) {
                Some((color, piece)) => piece.render_char(&color, &self.render_type),
                None => '\u{00B7}',
            };
        }

        println!("   a b c d e f g h");
        println!();
        for rank in (0..8).rev() {
            print!("{}  ", rank + 1);
            for file in 0..8 {
                let square = 8 * rank + file;

                print!("{} ", out_chars[square]);
            }
            println!(" {}", rank + 1);
        }
        println!();
        println!("   a b c d e f g h");
        println!();
    }
}

#[derive(Default)]
pub struct Game {
    board_state: BoardState,
}

impl Game {
    pub fn new() -> Self {
        let mut result = Self::default();
        let board_state = BoardState::new();
        result.board_state = board_state;
        result
    }

    pub fn board_state(&self) -> &BoardState {
        &self.board_state
    }

    pub fn natural_apply_move(&mut self, move_str: &str) -> Result<(), ChersError> {
        let chars = move_str.chars().collect::<Vec<char>>();

        if chars.len() < 4 {
            return Err(ChersError::MoveParseError);
        }

        let file_from_char = |c: char| match c {
            'a' => Ok(0),
            'b' => Ok(1),
            'c' => Ok(2),
            'd' => Ok(3),
            'e' => Ok(4),
            'f' => Ok(5),
            'g' => Ok(6),
            'h' => Ok(7),
            _ => Err(ChersError::MoveParseError),
        };

        let rank_from_char = |c: char| match c {
            '1' => Ok(0),
            '2' => Ok(1),
            '3' => Ok(2),
            '4' => Ok(3),
            '5' => Ok(4),
            '6' => Ok(5),
            '7' => Ok(6),
            '8' => Ok(7),
            _ => Err(ChersError::MoveParseError),
        };

        let mut promotion_piece = None;

        if chars.len() >= 5 {
            promotion_piece = match chars[5] {
                'q' => Some(PieceType::Queen),
                'r' => Some(PieceType::Rook),
                'k' => Some(PieceType::Knight),
                'b' => Some(PieceType::Bishop),
                _ => None,
            }
        }

        let file_1 = file_from_char(chars[0])?;
        let file_2 = file_from_char(chars[2])?;
        let rank_1 = rank_from_char(chars[1])?;
        let rank_2 = rank_from_char(chars[3])?;

        self.board_state
            .apply_move_by_coords(file_1, rank_1, file_2, rank_2, promotion_piece)?;

        Ok(())
    }

    pub fn apply_move_by_coords(
        &mut self,
        file_1: usize,
        rank_1: usize,
        file_2: usize,
        rank_2: usize,
        promotion_piece: Option<PieceType>,
    ) -> Result<(), ChersError> {
        self.board_state
            .apply_move_by_coords(file_1, rank_1, file_2, rank_2, promotion_piece)?;

        Ok(())
    }
}

#[derive(Default)]
pub struct BoardState {
    // bit masks for white pieces
    pieces: [u64; 12],

    //  0 en pessant col
    //  1 en pessant col
    //  2 en pessant col
    //  3 en pessant available
    //  4 castle right (wk)
    //  5 castle right (wq)
    //  6 castle right (bk)
    //  7 castle right (bq)
    special_moves: u8,

    white_to_move: bool,
}
impl BoardState {
    const WHITE_PAWNS_START: u64 = 0b11111111 << 8 * 1;
    const WHITE_ROOKS_START: u64 = 0b10000001 << 8 * 0;
    const WHITE_KNIGHTS_START: u64 = 0b01000010 << 8 * 0;
    const WHITE_BISHOPS_START: u64 = 0b00100100 << 8 * 0;
    const WHITE_QUEENS_START: u64 = 0b00001000 << 8 * 0;
    const WHITE_KINGS_START: u64 = 0b00010000 << 8 * 0;
    const BLACK_PAWNS_START: u64 = 0b11111111 << 8 * 6;
    const BLACK_ROOKS_START: u64 = 0b10000001 << 8 * 7;
    const BLACK_KNIGHTS_START: u64 = 0b01000010 << 8 * 7;
    const BLACK_BISHOPS_START: u64 = 0b00100100 << 8 * 7;
    const BLACK_QUEENS_START: u64 = 0b00001000 << 8 * 7;
    const BLACK_KINGS_START: u64 = 0b00010000 << 8 * 7;

    const WHITE_PAWNS_IDX: usize = 0;
    const WHITE_ROOKS_IDX: usize = 1;
    const WHITE_KNIGHTS_IDX: usize = 2;
    const WHITE_BISHOPS_IDX: usize = 3;
    const WHITE_QUEENS_IDX: usize = 4;
    const WHITE_KINGS_IDX: usize = 5;
    const BLACK_PAWNS_IDX: usize = 0;
    const BLACK_ROOKS_IDX: usize = 1;
    const BLACK_KNIGHTS_IDX: usize = 2;
    const BLACK_BISHOPS_IDX: usize = 3;
    const BLACK_QUEENS_IDX: usize = 4;
    const BLACK_KINGS_IDX: usize = 5;

    pub fn new() -> Self {
        Self {
            pieces: [
                Self::WHITE_PAWNS_START,
                Self::WHITE_ROOKS_START,
                Self::WHITE_KNIGHTS_START,
                Self::WHITE_BISHOPS_START,
                Self::WHITE_QUEENS_START,
                Self::WHITE_KINGS_START,
                Self::BLACK_PAWNS_START,
                Self::BLACK_ROOKS_START,
                Self::BLACK_KNIGHTS_START,
                Self::BLACK_BISHOPS_START,
                Self::BLACK_QUEENS_START,
                Self::BLACK_KINGS_START,
            ],
            special_moves: 0b11110000,
            white_to_move: true,
        }
    }

    pub fn apply_move_by_squares(
        &mut self,
        square_1: usize,
        square_2: usize,
        promotion_piece: Option<PieceType>,
    ) -> Result<(), ChersError> {

        match promotion_piece {
            None => {
                let (pieces_idxs, other_pieces_idxs) = if self.white_to_move {
                    (0..6, 6..12)
                } else {
                    (6..12, 0..6)
                };

                let move_mask = (0b1_u64 << square_1) | (0b1_u64 << square_2);
                let capture_mask = 0b1_u64 << square_2;

                for mask in self.pieces[pieces_idxs].iter_mut() {
                    if Self::square_occupied_by(square_2, *mask) {
                        *mask = *mask ^ capture_mask;
                        break;
                    }
                }
                for mask in self.pieces[other_pieces_idxs].iter_mut() {
                    if Self::square_occupied_by(square_1, *mask) {
                        *mask = *mask ^ move_mask;
                        break;
                    }
                }
            }
            Some(promotion_piece) => {
                let (pieces_idx, other_pieces_idxs) = if self.white_to_move {
                    (Self::WHITE_PAWNS_IDX, Self::BLACK_PAWNS_IDX..=Self::BLACK_KINGS_IDX)
                } else {
                    (Self::BLACK_PAWNS_IDX, Self::WHITE_PAWNS_IDX..=Self::WHITE_KINGS_IDX)
                };

                let move_mask = 0b1_u64 << square_1;
                let promo_mask = 0b1_u64 << square_2;
                let promo_idx = match (self.white_to_move, promotion_piece) {
                    (true, PieceType::Queen) => Ok(Self::WHITE_QUEENS_IDX),
                    (false, PieceType::Queen) => Ok(Self::BLACK_QUEENS_IDX),
                    (true, PieceType::Rook) => Ok(Self::WHITE_ROOKS_IDX),
                    (false, PieceType::Rook) => Ok(Self::BLACK_ROOKS_IDX),
                    (true, PieceType::Knight) => Ok(Self::WHITE_KNIGHTS_IDX),
                    (false, PieceType::Knight) => Ok(Self::BLACK_KNIGHTS_IDX),
                    (true, PieceType::Bishop) => Ok(Self::WHITE_BISHOPS_IDX),
                    (false, PieceType::Bishop) => Ok(Self::BLACK_BISHOPS_IDX),
                    _ => Err(ChersError::MoveParseError),
                }?;

                self.pieces[pieces_idx] = self.pieces[pieces_idx] ^ move_mask;
                self.pieces[promo_idx] = self.pieces[pieces_idx] ^ promo_mask;

                // todo - optimize by checking for diagonals
                let capture_mask = 0b1_u64 << square_2;
                for mask in self.pieces[other_pieces_idxs].iter_mut() {
                    if Self::square_occupied_by(square_1, *mask) {
                        *mask = *mask ^ capture_mask;
                        break;
                    }
                }
            }
        }

        self.white_to_move = !self.white_to_move;
        Ok(())
    }

    pub fn apply_move_by_coords(
        &mut self,
        file_1: usize,
        rank_1: usize,
        file_2: usize,
        rank_2: usize,
        promotion_piece: Option<PieceType>,
    ) -> Result<(), ChersError> {
        let square_1 = Self::coordinate_to_square(file_1, rank_1);
        let square_2 = Self::coordinate_to_square(file_2, rank_2);

        self.apply_move_by_squares(square_1, square_2, promotion_piece)?;
        Ok(())
    }

    pub fn coordinate_to_square(file: usize, rank: usize) -> usize {
        8 * rank + file
    }
    pub fn square_to_coordinate(square: usize) -> (usize, usize) {
        (square % 8, square / 8)
    }

    pub fn coordinate_occupied_by(file: usize, rank: usize, occupancy_mask: u64) -> bool {
        let square = Self::coordinate_to_square(file, rank);
        Self::square_occupied_by(square, occupancy_mask)
    }
    pub fn square_occupied_by(square: usize, occupancy_mask: u64) -> bool {
        ((0b1 << square) & occupancy_mask) != 0
    }

    pub fn piece_in_square(&self, square: usize) -> Option<(PieceColor, PieceType)> {
        if Self::square_occupied_by(square, self.white_pawns()) {
            Some((PieceColor::White, PieceType::Pawn))
        } else if Self::square_occupied_by(square, self.white_knights()) {
            Some((PieceColor::White, PieceType::Knight))
        } else if Self::square_occupied_by(square, self.white_bishops()) {
            Some((PieceColor::White, PieceType::Bishop))
        } else if Self::square_occupied_by(square, self.white_rooks()) {
            Some((PieceColor::White, PieceType::Rook))
        } else if Self::square_occupied_by(square, self.white_queens()) {
            Some((PieceColor::White, PieceType::Queen))
        } else if Self::square_occupied_by(square, self.white_kings()) {
            Some((PieceColor::White, PieceType::King))
        } else if Self::square_occupied_by(square, self.black_pawns()) {
            Some((PieceColor::Black, PieceType::Pawn))
        } else if Self::square_occupied_by(square, self.black_knights()) {
            Some((PieceColor::Black, PieceType::Knight))
        } else if Self::square_occupied_by(square, self.black_bishops()) {
            Some((PieceColor::Black, PieceType::Bishop))
        } else if Self::square_occupied_by(square, self.black_rooks()) {
            Some((PieceColor::Black, PieceType::Rook))
        } else if Self::square_occupied_by(square, self.black_queens()) {
            Some((PieceColor::Black, PieceType::Queen))
        } else if Self::square_occupied_by(square, self.black_kings()) {
            Some((PieceColor::Black, PieceType::King))
        } else {
            None
        }
    }

    pub fn white_occupancy(&self) -> u64 {
        self.white_pawns()
            | self.white_knights()
            | self.white_bishops()
            | self.white_rooks()
            | self.white_queens()
            | self.white_kings()
    }
    pub fn black_occupancy(&self) -> u64 {
        self.black_pawns()
            | self.black_knights()
            | self.black_bishops()
            | self.black_rooks()
            | self.black_queens()
            | self.black_kings()
    }
    pub fn total_occupancy(&self) -> u64 {
        self.white_occupancy() | self.black_occupancy()
    }

    pub fn white_pawns(&self) -> u64 {
        self.pieces[Self::WHITE_PAWNS_IDX]
    }
    pub fn white_rooks(&self) -> u64 {
        self.pieces[Self::WHITE_ROOKS_IDX]
    }
    pub fn white_knights(&self) -> u64 {
        self.pieces[Self::WHITE_KNIGHTS_IDX]
    }
    pub fn white_bishops(&self) -> u64 {
        self.pieces[Self::WHITE_BISHOPS_IDX]
    }
    pub fn white_queens(&self) -> u64 {
        self.pieces[Self::WHITE_QUEENS_IDX]
    }
    pub fn white_kings(&self) -> u64 {
        self.pieces[Self::WHITE_KINGS_IDX]
    }

    pub fn black_pawns(&self) -> u64 {
        self.pieces[Self::BLACK_PAWNS_IDX]
    }
    pub fn black_rooks(&self) -> u64 {
        self.pieces[Self::BLACK_ROOKS_IDX]
    }
    pub fn black_knights(&self) -> u64 {
        self.pieces[Self::BLACK_KNIGHTS_IDX]
    }
    pub fn black_bishops(&self) -> u64 {
        self.pieces[Self::BLACK_BISHOPS_IDX]
    }
    pub fn black_queens(&self) -> u64 {
        self.pieces[Self::BLACK_QUEENS_IDX]
    }
    pub fn black_kings(&self) -> u64 {
        self.pieces[Self::BLACK_KINGS_IDX]
    }
}
