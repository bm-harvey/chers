use crate::chess::Game;
use crate::core::{PieceColor, PieceType};
use colored::Colorize;

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

impl GameViewer {
    pub fn new() -> Self {
        Self {
            render_type: RenderType::ASCII,
        }
    }
    pub fn with_render_type(mut self, render_type: RenderType) -> Self {
        self.render_type = render_type;
        self
    }

    pub fn string_from_piece(&self, piece_type:&PieceType,piece_color: &PieceColor) -> String {
        self.char_from_piece(piece_type, piece_color).to_string()
    }
    fn char_from_piece(&self, piece_type:&PieceType, piece_color:&PieceColor)-> char{
        let (white_symbols, black_symbols) = match self.render_type {
            RenderType::ASCII => {
                return match piece_color {
                    PieceColor::White => match piece_type {
                        PieceType::Pawn => 'P',
                        PieceType::Rook => 'R',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Queen => 'Q',
                        PieceType::King => 'K',
                    },
                    PieceColor::Black => match piece_type {
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
        let idx = match piece_type {
            PieceType::Pawn => 0,
            PieceType::Rook => 1,
            PieceType::Knight => 2,
            PieceType::Bishop => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        };
        match piece_color {
            PieceColor::White => white_symbols[idx],
            PieceColor::Black => black_symbols[idx],
        }

    }

    pub fn print_square_idxs(&self) -> () {
        println!("    a  b  c  d  e  f  g  h");
        println!();
        for rank in (0..8).rev() {
            print!("{}  ", rank + 1);
            for file in 0..8 {
                let square = 8 * rank + file;

                if square < 10 {
                    print!("{}  ", square);
                } else {
                    print!("{} ", square);
                }
            }
            println!("  {}", rank + 1);
        }
        println!();
        println!("    a  b  c  d  e  f  g  h");
        println!();
    }

    pub fn plain_print(&self, game: &Game) -> () {
        let mut out_chars = ['*'; 64];

        for square in 0..64 {
            out_chars[square] = match game.board_state().piece_in_square(square) {
                Some((color, piece)) => self.char_from_piece(&piece, &color),
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

    pub fn print_legal_moves(&self, game: &Game, square: usize) -> () {
        // todo - fix piece detection
        let legal_squares = game.board_state().queen_movement_allowed_mask(square);

        let mut out_strings = vec![String::from("\u{2820}"); 64];

        for square in 0..64_usize {
            let square_valid = (legal_squares & (0b1_u64 << square)) != 0;

            out_strings[square as usize] = match game.board_state().piece_in_square(square) {
                Some((color, piece)) => {
                    let raw_string = self.string_from_piece(&piece, &color);
                    if square_valid {
                        raw_string.purple().to_string()
                    } else {
                        match color {
                            PieceColor::White => raw_string.red().to_string(),
                            PieceColor::Black => raw_string.blue().to_string(),
                        }
                    }
                }
                None => {
                    if square_valid {
                        "\u{2820}".purple().to_string()
                    } else {
                        "\u{2820}".to_string()
                    }
                }
            };
        }

        println!("   a b c d e f g h");
        println!();
        for rank in (0..8).rev() {
            print!("{}  ", rank + 1);
            for file in 0..8 {
                let square = 8 * rank + file;

                print!("{} ", out_strings[square]);
            }
            println!(" {}", rank + 1);
        }
        println!();
        println!("   a b c d e f g h");
        println!();
    }
    pub fn print(&self, game: &Game) -> () {
        let mut out_strings = vec![String::from("\u{2820}"); 64];

        for square in 0..64_usize {
            out_strings[square] = match game.board_state().piece_in_square(square) {
                Some((color, piece)) => {
                    let raw_string = self.string_from_piece(&piece, &PieceColor::White);
                    match color {
                        PieceColor::White => raw_string.red().to_string(),
                        PieceColor::Black => raw_string.blue().to_string(),
                    }
                }
                None => String::from("\u{2820}"),
            };
        }

        println!("   a b c d e f g h");
        println!();
        for rank in (0..8).rev() {
            print!("{}  ", rank + 1);
            for file in 0..8 {
                let square = 8 * rank + file;

                print!("{} ", out_strings[square]);
            }
            println!(" {}", rank + 1);
        }
        println!();
        println!("   a b c d e f g h");
        println!();
    }
}
