pub enum PieceType {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
    Empty,
}

impl PieceType {
    pub fn to_char(&self) -> char {
        match self {
            PieceType::WhitePawn => '\u{2659}',
            PieceType::WhiteRook => '\u{2656}',
            PieceType::WhiteKnight => '\u{2658}',
            PieceType::WhiteBishop => '\u{2657}',
            PieceType::WhiteQueen => '\u{2655}',
            PieceType::WhiteKing => '\u{2654}',
            PieceType::BlackPawn => '\u{265F}',
            PieceType::BlackRook => '\u{265C}',
            PieceType::BlackKnight => '\u{265E}',
            PieceType::BlackBishop => '\u{265D}',
            PieceType::BlackQueen => '\u{265B}',
            PieceType::BlackKing => '\u{265A}',
            PieceType::Empty => '\u{2810}',
        }
    }
}

#[derive(Default)]
pub struct GameState {
    board_state: BoardState,
}

impl GameState {
    pub fn new() -> Self {
        let mut result = Self::default();
        let board_state = BoardState::new();
        result.board_state = board_state;
        result
    }

    pub fn board_state(&self) -> &BoardState {
        &self.board_state
    }

    pub fn apply_move(&mut self, file_1: usize, rank_1: usize, file_2: usize, rank_2: usize) {
        self.board_state.apply_move(file_1, rank_1, file_2, rank_2);
    }

    pub fn print(&self) -> () {
        let mut out_chars = ['*'; 64];

        for square in 0..64 {
            out_chars[square] = self.board_state.piece_in_square(square).to_char()
        }

        println!("  0 1 2 3 4 5 6 7");
        for rank in (0..8).rev() {
            print!("{} ", rank);
            for file in 0..8 {
                let square = 8 * rank + file;

                print!("{} ", out_chars[square]);
            }
            println!(" {}", rank);
        }
        println!("  0 1 2 3 4 5 6 7");
        println!();
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
            white_to_move : true,
        }
    }

    pub fn apply_move(&mut self, file_1: usize, rank_1: usize, file_2: usize, rank_2: usize) {
        let square_1 = Self::coordinate_to_square(file_1, rank_1);
        let square_2 = Self::coordinate_to_square(file_2, rank_2);

        let move_mask = (0b1_u64 << square_1) | (0b1_u64 << square_2);
        let capture_mask = 0b1_u64 << square_2;

        for mask in self.pieces.iter_mut() {
            if Self::square_occupied_by(square_2, *mask) {
                *mask = *mask ^ capture_mask;
                break;
            }
        }
        for mask in self.pieces.iter_mut() {
            if Self::square_occupied_by(square_1, *mask) {
                *mask = *mask ^ move_mask;
                break;
            }
        }
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

    pub fn piece_in_square(&self, square: usize) -> PieceType {
        if Self::square_occupied_by(square, self.white_pawns()) {
            PieceType::WhitePawn
        } else if Self::square_occupied_by(square, self.white_knights()) {
            PieceType::WhiteKnight
        } else if Self::square_occupied_by(square, self.white_bishops()) {
            PieceType::WhiteBishop
        } else if Self::square_occupied_by(square, self.white_rooks()) {
            PieceType::WhiteRook
        } else if Self::square_occupied_by(square, self.white_queens()) {
            PieceType::WhiteQueen
        } else if Self::square_occupied_by(square, self.white_kings()) {
            PieceType::WhiteKing
        } else if Self::square_occupied_by(square, self.black_pawns()) {
            PieceType::BlackPawn
        } else if Self::square_occupied_by(square, self.black_knights()) {
            PieceType::BlackKnight
        } else if Self::square_occupied_by(square, self.black_bishops()) {
            PieceType::BlackBishop
        } else if Self::square_occupied_by(square, self.black_rooks()) {
            PieceType::BlackRook
        } else if Self::square_occupied_by(square, self.black_queens()) {
            PieceType::BlackQueen
        } else if Self::square_occupied_by(square, self.black_kings()) {
            PieceType::BlackKing
        } else {
            PieceType::Empty
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
        self.pieces[0]
    }
    pub fn white_rooks(&self) -> u64 {
        self.pieces[1]
    }
    pub fn white_knights(&self) -> u64 {
        self.pieces[2]
    }
    pub fn white_bishops(&self) -> u64 {
        self.pieces[3]
    }
    pub fn white_queens(&self) -> u64 {
        self.pieces[4]
    }
    pub fn white_kings(&self) -> u64 {
        self.pieces[5]
    }

    pub fn black_pawns(&self) -> u64 {
        self.pieces[6]
    }
    pub fn black_rooks(&self) -> u64 {
        self.pieces[7]
    }
    pub fn black_knights(&self) -> u64 {
        self.pieces[8]
    }
    pub fn black_bishops(&self) -> u64 {
        self.pieces[9]
    }
    pub fn black_queens(&self) -> u64 {
        self.pieces[10]
    }
    pub fn black_kings(&self) -> u64 {
        self.pieces[11]
    }
}
