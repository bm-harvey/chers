use crate::chess::{BoardState, Game};
use crate::core::{PieceColor, PieceType, bits};
use colored::Colorize;

#[derive(Default)]
pub enum RenderType {
    #[default]
    ASCII,
    ASCIIGreek,
    Normal,
    Inverted, // for dark terminals
}

pub struct GameViewer {
    render_type: RenderType,
}

impl Default for GameViewer {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn string_from_piece(&self, piece_type: &PieceType, piece_color: &PieceColor) -> String {
        self.char_from_piece(piece_type, piece_color).to_string()
    }
    fn char_from_piece(&self, piece_type: &PieceType, piece_color: &PieceColor) -> char {
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
            RenderType::ASCIIGreek => {
                return match piece_type {
                    PieceType::Pawn => '\u{03C0}',
                    PieceType::Rook => '\u{03A0}',
                    PieceType::Knight => '\u{0393}',
                    PieceType::Bishop => '\u{0394}',
                    PieceType::Queen => '\u{03C8}',
                    PieceType::King => '+',
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

    pub fn print_square_idxs(&self) {
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

    pub fn plain_print(&self, game: &Game) {
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

    pub fn print_board(labels: &[String]) {
        println!();
        println!("   a b c d e f g h");
        for rank in (0..8).rev() {
            print!("{}  ", rank + 1);
            for file in 0..8 {
                let square = 8 * rank + file;

                print!("{} ", labels[square]);
            }
            println!(" {}", rank + 1);
        }
        println!("   a b c d e f g h");
        println!();
    }

    pub fn print_legal_moves(&self, game: &Game) {
        // todo - fix piece detection
        //
        let start = std::time::Instant::now();
        let legal_squares = game.board_state().psuedo_legal_moves();
        let stop = start.elapsed().as_nanos();

        let blank = String::from("\u{25A0}");

        let mut default_out_strings = vec![blank.clone(); 64];

        for square in 0..64_usize {
            let (file, rank) = BoardState::square_to_coordinate(square);
            default_out_strings[square] = match game.board_state().piece_in_square(square) {
                Some((color, piece)) => {
                    let raw_string = self.string_from_piece(&piece, &PieceColor::Black);

                    match color {
                        PieceColor::White => raw_string.red().to_string(),
                        PieceColor::Black => raw_string.blue().to_string(),
                    }
                }

                None => {
                    if (file + rank) % 2 == 1 {
                        blank.clone().white().to_string()
                    } else {
                        blank.clone().truecolor(100, 100, 100).to_string()
                    }
                }
            };
        }
        for move_set in legal_squares.iter() {
            let mut highlights = vec![bits::square_mask_to_idx(move_set.0)];
            for destination in bits::Biterator::new(move_set.1) {
                highlights.push(bits::square_mask_to_idx(destination))
            }

            let mut out_strings = default_out_strings.clone();
            for highlight in highlights {
                let current_string = default_out_strings[highlight].clone();

                out_strings[highlight] =
                    if (game.board_state().total_occupancy() & (0b1_u64 << highlight)) == 0 {
                        blank.purple().to_string()
                    } else {
                        current_string.bold().underline().italic().to_string()
                    }
            }
            GameViewer::print_board(&out_strings);
        }

        dbg!(stop);
    }
    pub fn print(&self, game: &Game) {
        let blank = String::from("\u{25A0}");

        let mut default_out_strings = vec![blank.clone(); 64];

        for square in 0..64_usize {
            let (file, rank) = BoardState::square_to_coordinate(square);
            default_out_strings[square] = match game.board_state().piece_in_square(square) {
                Some((color, piece)) => {
                    let raw_string = self.string_from_piece(&piece, &PieceColor::Black);

                    match color {
                        PieceColor::White => raw_string.red().to_string(),
                        PieceColor::Black => raw_string.blue().to_string(),
                    }
                }

                None => {
                    if (file + rank) % 2 == 1 {
                        blank.clone().white().to_string()
                    } else {
                        blank.clone().truecolor(100, 100, 100).to_string()
                    }
                }
            };
        }
        GameViewer::print_board(&default_out_strings);
    }
}
