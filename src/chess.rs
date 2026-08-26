use crate::core::bits::{self, Biterator, square_idx_to_mask};
use crate::core::{ChersError, PieceColor, PieceType};
use rand::RngExt;
use rand::seq::IndexedRandom;
use smallvec::{SmallVec, smallvec};
use std::char;

#[derive(Default)]
pub struct Game {
    board_states: Vec<BoardState>,
}

impl Game {
    pub fn new() -> Self {
        let mut result = Self::default();
        let board_state = BoardState::new();
        result.board_states = vec![board_state];
        result
    }

    pub fn board_state(&self) -> &BoardState {
        &self.board_states.last().unwrap()
    }

    pub fn do_random_pseudo_legal_move(&mut self) -> Result<(), ChersError> {
        let move_squares = self.board_state().random_psuedo_legal_move();


        if move_squares.0 == 0 || move_squares.1 == 0 {
            Err(ChersError::IllegalMoveError)
        } else {
            self.apply_move_by_squares(
                bits::square_mask_to_idx(move_squares.0),
                bits::square_mask_to_idx(move_squares.1),
                None,
            )
        }
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

        self.apply_move_by_coords(file_1, rank_1, file_2, rank_2, promotion_piece)?;

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
        self.board_states
            .push(self.board_state().generate_board_after_move(
                file_1,
                rank_1,
                file_2,
                rank_2,
                promotion_piece,
            )?);

        Ok(())
    }
    pub fn apply_move_by_squares(
        &mut self,
        square_1: usize,
        square_2: usize,
        promotion_piece: Option<PieceType>,
    ) -> Result<(), ChersError> {
        self.board_states
            .push(self.board_state().generate_board_after_move_by_squares(
                square_1,
                square_2,
                promotion_piece,
            )?);

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct BoardState {
    // bit masks for pieces
    pieces: [u64; 12],

    //  0 en passant col
    //  1 en passant col
    //  2 en passant col
    //  3 en passant available
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
    const BLACK_PAWNS_IDX: usize = 6;
    const BLACK_ROOKS_IDX: usize = 7;
    const BLACK_KNIGHTS_IDX: usize = 8;
    const BLACK_BISHOPS_IDX: usize = 9;
    const BLACK_QUEENS_IDX: usize = 10;
    const BLACK_KINGS_IDX: usize = 11;
}

impl BoardState {
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

    pub fn en_passant_square_mask(&self) -> u64 {
        if self.special_moves & (0b1_u8 << 3) != 0 {
            let file = (self.special_moves & (0b00000111)) as usize;
            let rank = if self.white_to_move { 5 } else { 2 };

            let square_idx = BoardState::coordinate_to_square(file, rank);

            bits::square_idx_to_mask(square_idx)
        } else {
            0_u64
        }
    }

    pub fn generate_board_after_move_by_squares(
        &self,
        square_1: usize,
        square_2: usize,
        promotion_piece: Option<PieceType>,
    ) -> Result<BoardState, ChersError> {
        let mut result = self.clone();
        result.special_moves &= 0b11110000;
        match promotion_piece {
            None => {
                let (pieces_idxs, other_pieces_idxs) = if self.white_to_move {
                    (
                        Self::WHITE_PAWNS_IDX..=Self::WHITE_KINGS_IDX,
                        Self::BLACK_PAWNS_IDX..=Self::BLACK_PAWNS_IDX,
                    )
                } else {
                    (
                        Self::BLACK_PAWNS_IDX..=Self::BLACK_KINGS_IDX,
                        Self::WHITE_PAWNS_IDX..=Self::WHITE_PAWNS_IDX,
                    )
                };

                let move_mask = (0b1_u64 << square_1) | (0b1_u64 << square_2);
                let capture_mask = 0b1_u64 << square_2;

                for (idx, mask) in result.pieces[pieces_idxs].iter_mut().enumerate() {
                    if Self::square_occupied_by(square_1, *mask) {
                        *mask = *mask ^ move_mask;
                        if idx == 0 && square_1.abs_diff(square_2) == 16 {
                            // enable en passant
                            let (file, _) = BoardState::square_to_coordinate(square_2);
                            let file = file as u8;
                            let new_bits = (0b1_u8 << 3) | file;
                            result.special_moves |= new_bits;
                        }
                        break;
                    }
                }
                for mask in result.pieces[other_pieces_idxs].iter_mut() {
                    if Self::square_occupied_by(square_2, *mask) {
                        *mask = *mask ^ capture_mask;
                        break;
                    }
                }
            }
            Some(promotion_piece) => {
                let (pieces_idx, other_pieces_idxs) = if self.white_to_move {
                    (
                        Self::WHITE_PAWNS_IDX,
                        Self::BLACK_PAWNS_IDX..=Self::BLACK_KINGS_IDX,
                    )
                } else {
                    (
                        Self::BLACK_PAWNS_IDX,
                        Self::WHITE_PAWNS_IDX..=Self::WHITE_KINGS_IDX,
                    )
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

                result.pieces[pieces_idx] = self.pieces[pieces_idx] ^ move_mask;
                result.pieces[promo_idx] = self.pieces[pieces_idx] ^ promo_mask;

                // todo - optimize by checking for diagonals
                let capture_mask = 0b1_u64 << square_2;
                for mask in result.pieces[other_pieces_idxs].iter_mut() {
                    if Self::square_occupied_by(square_1, *mask) {
                        *mask = *mask ^ capture_mask;
                        break;
                    }
                }
            }
        }

        result.white_to_move = !self.white_to_move;
        Ok(result)
    }

    pub fn generate_board_after_move(
        &self,
        file_1: usize,
        rank_1: usize,
        file_2: usize,
        rank_2: usize,
        promotion_piece: Option<PieceType>,
    ) -> Result<BoardState, ChersError> {
        let square_1 = Self::coordinate_to_square(file_1, rank_1);
        let square_2 = Self::coordinate_to_square(file_2, rank_2);

        Ok(self.generate_board_after_move_by_squares(square_1, square_2, promotion_piece)?)
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
        ((0b1_u64 << square) & occupancy_mask) != 0
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

    pub fn active_pawns(&self) -> u64 {
        if self.white_to_move {
            self.white_pawns()
        } else {
            self.black_pawns()
        }
    }
    pub fn active_rooks(&self) -> u64 {
        if self.white_to_move {
            self.white_rooks()
        } else {
            self.black_rooks()
        }
    }
    pub fn active_knights(&self) -> u64 {
        if self.white_to_move {
            self.white_knights()
        } else {
            self.black_knights()
        }
    }
    pub fn active_bishops(&self) -> u64 {
        if self.white_to_move {
            self.white_bishops()
        } else {
            self.black_bishops()
        }
    }
    pub fn active_queens(&self) -> u64 {
        if self.white_to_move {
            self.white_queens()
        } else {
            self.black_queens()
        }
    }
    pub fn active_kings(&self) -> u64 {
        if self.white_to_move {
            self.white_kings()
        } else {
            self.black_kings()
        }
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

    pub fn material_value(&self) -> f32 {
        (self.white_pawns().count_ones() as f32) * 1_f32
            + (self.white_rooks().count_ones() as f32) * 5_f32
            + (self.white_knights().count_ones() as f32) * 3_f32
            + (self.white_bishops().count_ones() as f32) * 3_f32
            + (self.white_queens().count_ones() as f32) * 9_f32
            - (self.black_pawns().count_ones() as f32) * 1_f32
            - (self.black_rooks().count_ones() as f32) * 5_f32
            - (self.black_knights().count_ones() as f32) * 3_f32
            - (self.black_bishops().count_ones() as f32) * 3_f32
            - (self.black_queens().count_ones() as f32) * 9_f32
    }
    pub fn num_active_pieces(&self) -> u32 {
        if self.white_to_move {
            self.white_occupancy().count_ones()
        } else {
            self.black_occupancy().count_ones()
        }
    }
}

impl BoardState {
    pub fn queen_pseudo_allowed_moves(&self, starting_square_mask: u64) -> u64 {
        self.rook_pseudo_allowed_moves(starting_square_mask)
            | self.bishop_pseudo_allowed_moves(starting_square_mask)
    }

    pub fn rook_pseudo_allowed_moves(&self, starting_square_mask: u64) -> u64 {
        let mut mask = 0b0_u64;

        let mut own_occupancy = self.white_occupancy();
        let mut other_occupancy = self.black_occupancy();
        if !self.white_to_move {
            std::mem::swap(&mut own_occupancy, &mut other_occupancy);
        }

        let mut add_destination_and_should_break = |destination_mask: u64| {
            if (own_occupancy & destination_mask) != 0 {
                true
            } else if (other_occupancy & destination_mask) != 0 {
                mask |= destination_mask;
                true
            } else {
                mask |= destination_mask;
                false
            }
        };

        let mut current_square_mask = starting_square_mask;
        while bits::square_exists_up_mask(current_square_mask) {
            current_square_mask = bits::move_one_up(current_square_mask);
            if add_destination_and_should_break(current_square_mask) {
                break;
            }
        }
        let mut current_square_mask = starting_square_mask;
        while bits::square_exists_down_mask(current_square_mask) {
            current_square_mask = bits::move_one_down(current_square_mask);
            if add_destination_and_should_break(current_square_mask) {
                break;
            }
        }
        let mut current_square_mask = starting_square_mask;
        while bits::square_exists_left_mask(current_square_mask) {
            current_square_mask = bits::move_one_left(current_square_mask);
            if add_destination_and_should_break(current_square_mask) {
                break;
            }
        }
        let mut current_square_mask = starting_square_mask;
        while bits::square_exists_right_mask(current_square_mask) {
            current_square_mask = bits::move_one_right(current_square_mask);
            if add_destination_and_should_break(current_square_mask) {
                break;
            }
        }

        mask
    }

    pub fn bishop_pseudo_allowed_moves(&self, starting_square_mask: u64) -> u64 {
        let mut mask = 0b0_u64;

        let mut own_occupancy = self.white_occupancy();
        let mut other_occupancy = self.black_occupancy();
        if !self.white_to_move {
            std::mem::swap(&mut own_occupancy, &mut other_occupancy);
        }

        let mut add_destination_and_should_break = |destination_mask: u64| {
            if (own_occupancy & destination_mask) != 0 {
                true
            } else if (other_occupancy & destination_mask) != 0 {
                mask |= destination_mask;
                true
            } else {
                mask |= destination_mask;
                false
            }
        };

        let mut current_square_mask = starting_square_mask;
        while bits::square_exists_up_mask(current_square_mask)
            && bits::square_exists_left_mask(current_square_mask)
        {
            current_square_mask = bits::move_one_up_one_left(current_square_mask);
            if add_destination_and_should_break(current_square_mask) {
                break;
            }
        }

        let mut current_square_mask = starting_square_mask;
        while bits::square_exists_down_mask(current_square_mask)
            && bits::square_exists_left_mask(current_square_mask)
        {
            current_square_mask = bits::move_one_down_one_left(current_square_mask);
            if add_destination_and_should_break(current_square_mask) {
                break;
            }
        }

        let mut current_square_mask = starting_square_mask;
        while bits::square_exists_up_mask(current_square_mask)
            && bits::square_exists_right_mask(current_square_mask)
        {
            current_square_mask = bits::move_one_up_one_right(current_square_mask);
            if add_destination_and_should_break(current_square_mask) {
                break;
            }
        }

        let mut current_square_mask = starting_square_mask;
        while bits::square_exists_down_mask(current_square_mask)
            && bits::square_exists_right_mask(current_square_mask)
        {
            current_square_mask = bits::move_one_down_one_right(current_square_mask);
            if add_destination_and_should_break(current_square_mask) {
                break;
            }
        }

        mask
    }

    pub fn knight_pseudo_allowed_moves(&self, starting_square_mask: u64) -> u64 {
        let mut mask = 0b0_u64;

        let own_occupancy = if self.white_to_move {
            self.white_occupancy()
        } else {
            self.black_occupancy()
        };

        let current_square_mask = starting_square_mask;

        let mut add_destination = |target_mask: u64| {
            if target_mask & own_occupancy == 0 {
                mask |= target_mask;
            }
        };

        if bits::square_exists_two_up_mask(current_square_mask)
            && bits::square_exists_left_mask(current_square_mask)
        {
            add_destination(bits::move_two_up_one_left(current_square_mask));
        }
        if bits::square_exists_two_up_mask(current_square_mask)
            && bits::square_exists_right_mask(current_square_mask)
        {
            add_destination(bits::move_two_up_one_right(current_square_mask));
        }
        if bits::square_exists_up_mask(current_square_mask)
            && bits::square_exists_two_left_mask(current_square_mask)
        {
            add_destination(bits::move_one_up_two_left(current_square_mask));
        }
        if bits::square_exists_up_mask(current_square_mask)
            && bits::square_exists_two_right_mask(current_square_mask)
        {
            add_destination(bits::move_one_up_two_right(current_square_mask));
        }

        if bits::square_exists_two_down_mask(current_square_mask)
            && bits::square_exists_left_mask(current_square_mask)
        {
            add_destination(bits::move_two_down_one_left(current_square_mask));
        }

        if bits::square_exists_two_down_mask(current_square_mask)
            && bits::square_exists_right_mask(current_square_mask)
        {
            add_destination(bits::move_two_down_one_right(current_square_mask));
        }
        if bits::square_exists_down_mask(current_square_mask)
            && bits::square_exists_two_left_mask(current_square_mask)
        {
            add_destination(bits::move_one_down_two_left(current_square_mask));
        }
        if bits::square_exists_down_mask(current_square_mask)
            && bits::square_exists_two_right_mask(current_square_mask)
        {
            add_destination(bits::move_one_down_two_right(current_square_mask));
        }

        mask
    }

    pub fn king_pseudo_allowed_moves(&self, starting_square_mask: u64) -> u64 {
        let mut mask = 0b0_u64;

        let own_occupancy = if self.white_to_move {
            self.white_occupancy()
        } else {
            self.black_occupancy()
        };

        let current_square_mask = starting_square_mask;

        let square_up = bits::square_exists_up_mask(current_square_mask);
        let square_down = bits::square_exists_down_mask(current_square_mask);
        let square_left = bits::square_exists_left_mask(current_square_mask);
        let square_right = bits::square_exists_right_mask(current_square_mask);

        let mut add_destination = |target_mask: u64| {
            if target_mask & own_occupancy == 0 {
                mask |= target_mask;
            }
        };

        if square_up {
            add_destination(bits::move_one_up(current_square_mask));

            if square_left {
                add_destination(bits::move_one_up_one_left(current_square_mask));
            }
            if square_right {
                add_destination(bits::move_one_up_one_right(current_square_mask));
            }
        }

        if square_down {
            add_destination(bits::move_one_down(current_square_mask));

            if square_left {
                add_destination(bits::move_one_down_one_left(current_square_mask));
            }
            if square_right {
                add_destination(bits::move_one_down_one_right(current_square_mask));
            }
        }

        if square_left {
            add_destination(bits::move_one_left(current_square_mask));
        }
        if square_right {
            add_destination(bits::move_one_right(current_square_mask));
        }

        mask
    }

    pub fn pawn_pseudo_allowed_moves(&self, starting_square_mask: u64) -> u64 {
        let mut mask = 0b0_u64;
        let current_square_mask = starting_square_mask;

        let (own_occupancy, other_occupancy, starting_mask) = if self.white_to_move {
            (
                self.white_occupancy(),
                self.black_occupancy(),
                0x00_00_00_00_00_00_ff_00_u64,
            )
        } else {
            (
                self.black_occupancy(),
                self.white_occupancy(),
                0x00_ff_00_00_00_00_00_00_u64,
            )
        };

        let move_forward = |square_mask: u64| {
            if self.white_to_move {
                bits::move_one_up(square_mask)
            } else {
                bits::move_one_down(square_mask)
            }
        };

        let forward_square = move_forward(current_square_mask);
        if forward_square & (own_occupancy | other_occupancy) == 0 {
            mask |= forward_square;

            if current_square_mask & starting_mask != 0 {
                let double_forward = move_forward(forward_square);
                if double_forward & (own_occupancy | other_occupancy) == 0 {
                    mask |= double_forward;
                }
            }
        }

        let en_passant_sqaure = self.en_passant_square_mask();

        if bits::square_exists_left_mask(forward_square) {
            let capture_left = bits::move_one_left(forward_square);

            if capture_left & (other_occupancy | en_passant_sqaure) != 0 {
                mask |= capture_left;
            }
        }

        if bits::square_exists_right_mask(forward_square) {
            let capture_right = bits::move_one_right(forward_square);
            if capture_right & (other_occupancy | en_passant_sqaure) != 0 {
                mask |= capture_right;
            }
        }

        mask
    }

    pub fn psuedo_legal_moves(&self) -> Vec<(u64, u64)> {
        let num_active_pieces = self.num_active_pieces();
        let mut result = Vec::<(u64, u64)>::with_capacity(num_active_pieces as usize);

        for pawn in bits::Biterator::new(self.active_pawns()) {
            result.push((pawn, self.pawn_pseudo_allowed_moves(pawn)))
        }
        for rook in bits::Biterator::new(self.active_rooks()) {
            result.push((rook, self.rook_pseudo_allowed_moves(rook)))
        }
        for knights in bits::Biterator::new(self.active_knights()) {
            result.push((knights, self.knight_pseudo_allowed_moves(knights)))
        }
        for bishops in bits::Biterator::new(self.active_bishops()) {
            result.push((bishops, self.bishop_pseudo_allowed_moves(bishops)))
        }
        for queens in bits::Biterator::new(self.active_queens()) {
            result.push((queens, self.queen_pseudo_allowed_moves(queens)))
        }
        let kings = self.active_kings();
        result.push((kings, self.king_pseudo_allowed_moves(kings)));

        result
    }
    pub fn random_psuedo_legal_move(&self) -> (u64, u64) {
        let pseudo_legal_moves = self.psuedo_legal_moves();

        let mut rng = rand::rng();
        let target =
            pseudo_legal_moves.choose_weighted(&mut rng, |(_targ, dest)| dest.count_ones());

        if let Ok(target) = target {
            let destination_choice = rng.random_range(0..target.1.count_ones());
            (
                target.0,
                Biterator::new(target.1)
                    .nth(destination_choice as usize)
                    .unwrap(),
            )
        } else {
            (0, 0)
        }
    }
}
