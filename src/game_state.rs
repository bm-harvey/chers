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
            PieceType::WhitePawn => 'P',
            PieceType::WhiteRook => 'R',
            PieceType::WhiteKnight => 'N',
            PieceType::WhiteBishop => 'B',
            PieceType::WhiteQueen => 'Q',
            PieceType::WhiteKing => 'K',
            PieceType::BlackPawn => 'p',
            PieceType::BlackRook => 'r',
            PieceType::BlackKnight => 'n',
            PieceType::BlackBishop => 'b',
            PieceType::BlackQueen => 'q',
            PieceType::BlackKing => 'k',
            PieceType::Empty => '*',
        }
    }
}

#[derive(Default)]
pub struct GameState {
    board_state: BoardState,

    extra_info: u16,
}

#[derive(Default)]
pub struct BoardState {
    // bit masks for white pieces
    w_pawns: u64,
    w_rooks: u64,
    w_knights: u64,
    w_bishops: u64,
    w_queens: u64,
    w_kings: u64,

    // bit masks for black pieces
    b_pawns: u64,
    b_rooks: u64,
    b_knights: u64,
    b_bishops: u64,
    b_queens: u64,
    b_kings: u64,

    //  0 en pessant col
    //  1 en pessant col
    //  2 en pessant col
    //  3 en pessant available
    //  4 castle right (wk)
    //  5 castle right (wq)
    //  6 castle right (bk)
    //  7 castle right (bq)
    special_moves: u8,
}

impl GameState {
    pub fn new() -> Self {
        dbg!();
        let mut result = Self::default();
        let board_state = BoardState::new();
        result.board_state = board_state;
        result
    }
    pub fn board_state(&self) -> &BoardState {
        &self.board_state
    }

    pub fn extra_info(&self) -> u16 {
        self.extra_info
    }

    pub fn print(&self) -> () {
        let mut out_chars = ['*'; 64];

        let occupancy = self.board_state.occupancy();

        dbg!(occupancy);

        for square in 0..64 {
            let mask: u64 = 0b1 << square;
            if (mask & occupancy) != 0 {
                out_chars[square] = 'X'
            }
        }

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = 8 * rank + file;
                let file = square % 8;

                print!("{} ", out_chars[square]);
                if file == 7 {
                    println!();
                }
            }
        }
    }
}
impl BoardState {
    const WHITE_PAWNS_START: u64 = 0b11111111 << 8 * 1;
    const WHITE_ROOKS_START: u64 = 0b10000001 << 8 * 0;
    const WHITE_KNIGHTS_START: u64 = 0b01000010 << 8 * 0;
    const WHITE_BISHOPS_START: u64 = 0b00100100 << 8 * 0;
    const WHITE_QUEENS_START: u64 = 0b00010000 << 8 * 0;
    const WHITE_KINGS_START: u64 = 0b00001000 << 8 * 0;
    const BLACK_PAWNS_START: u64 = 0b11111111 << 8 * 6;
    const BLACK_ROOKS_START: u64 = 0b10000001 << 8 * 7;
    const BLACK_KNIGHTS_START: u64 = 0b01000010 << 8 * 7;
    const BLACK_BISHOPS_START: u64 = 0b00100100 << 8 * 7;
    const BLACK_QUEENS_START: u64 = 0b00010000 << 8 * 7;
    const BLACK_KINGS_START: u64 = 0b00001000 << 8 * 7;

    pub fn new() -> Self {
        let w_pawns: u64 = Self::WHITE_PAWNS_START;
        let mut result = Self::default();
        result.w_pawns = w_pawns;
        return result;
    }

    pub fn square_occupied_by(square: usize, occupancy_mask: u64) -> bool {
        ((0b1 << square) & occupancy_mask) != 0
    }

    pub fn w_occupancy(&self) -> u64 {
        self.w_pawns | self.w_knights | self.w_rooks | self.w_queens | self.w_kings
    }
    pub fn b_occupancy(&self) -> u64 {
        self.b_pawns | self.b_knights | self.b_rooks | self.b_queens | self.b_kings
    }
    pub fn occupancy(&self) -> u64 {
        self.w_occupancy() | self.b_occupancy()
    }

    pub fn w_pawns(&self) -> u64 {
        self.w_pawns
    }
    pub fn w_rooks(&self) -> u64 {
        self.w_rooks
    }
    pub fn w_knights(&self) -> u64 {
        self.w_knights
    }
    pub fn w_bishops(&self) -> u64 {
        self.w_bishops
    }
    pub fn w_queens(&self) -> u64 {
        self.w_queens
    }
    pub fn w_kings(&self) -> u64 {
        self.w_kings
    }

    pub fn b_pawns(&self) -> u64 {
        self.b_pawns
    }
    pub fn b_rooks(&self) -> u64 {
        self.b_rooks
    }
    pub fn b_knights(&self) -> u64 {
        self.b_knights
    }
    pub fn b_bishops(&self) -> u64 {
        self.b_bishops
    }
    pub fn b_queens(&self) -> u64 {
        self.b_queens
    }
    pub fn b_kings(&self) -> u64 {
        self.b_kings
    }
}
