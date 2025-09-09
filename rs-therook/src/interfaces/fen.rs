use crate::engine::*;
use FenSection::*;
use itertools::Itertools;

// https://www.chess.com/terms/fen-chess

pub enum FenSection {
    PiecePlacement(i8, i8),
    ActiveColor(bool),
    CastlingRights(bool),
    PossibleEnPassantTargets(String),
    HalfMoveClock(String),
    FullMoveNumber(String),
    Finished,
}

pub struct FenError(String);

impl std::fmt::Debug for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Fen(String);

impl Fen {
    pub fn new(string: String) -> Self {
        Fen(string)
    }

    pub fn initial() -> Self {
        Fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into())
    }
}

impl std::fmt::Debug for Fen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&Board> for Fen {
    fn from(board: &Board) -> Self {
        timed!("dumping board to fen", {
            let mut fen = String::new();

            let mut stack = 0;

            for rank in (0..8u8).rev() {
                for file in 0..8u8 {
                    let index = (rank * 8) + file;

                    if let Some(piece) = board.squares[index as usize] {
                        if stack > 0 {
                            fen.push_str(&format!("{stack}"));
                            stack = 0;
                        }

                        match piece {
                            WHITE_KING => fen.push('K'),
                            WHITE_QUEEN => fen.push('Q'),
                            WHITE_ROOK => fen.push('R'),
                            WHITE_BISHOP => fen.push('B'),
                            WHITE_KNIGHT => fen.push('N'),
                            WHITE_PAWN => fen.push('P'),
                            BLACK_KING => fen.push('k'),
                            BLACK_QUEEN => fen.push('q'),
                            BLACK_ROOK => fen.push('r'),
                            BLACK_BISHOP => fen.push('b'),
                            BLACK_KNIGHT => fen.push('n'),
                            BLACK_PAWN => fen.push('p'),
                            _ => unreachable!(),
                        }
                    } else {
                        stack += 1;
                    }
                }

                if stack > 0 {
                    fen.push_str(&format!("{stack}"));
                    stack = 0;
                }

                fen.push('/');
            }

            fen.pop();
            fen.push(' ');

            match board.turn {
                PieceColor::White => fen.push('w'),
                PieceColor::Black => fen.push('b'),
            }

            fen.push(' ');

            if board.castling.can(CastlingDirection::WhiteKing) {
                fen.push('K');
            }

            if board.castling.can(CastlingDirection::WhiteQueen) {
                fen.push('Q');
            }

            if board.castling.can(CastlingDirection::BlackKing) {
                fen.push('k');
            }

            if board.castling.can(CastlingDirection::BlackQueen) {
                fen.push('q');
            }

            if u8::from(&board.castling) == 0 {
                fen.push('-');
            }

            fen.push(' ');

            if board.enpassant.is_some() {
                let tile = board.enpassant.into_iter().find_or_first(|_| true).unwrap();
                fen.push_str(&format!("{tile:?}"));
            } else {
                fen.push('-');
            }

            fen.push(' ');

            fen.push_str(&format!("{}", board.halfmove));

            fen.push(' ');

            fen.push_str(&format!("{}", board.fullmove));

            Fen(fen)
        })
    }
}

impl TryInto<Board> for Fen {
    type Error = FenError;

    fn try_into(self) -> Result<Board, FenError> {
        timed!("parsing fen into board", {
            let mut board = Board::new();
            let mut section = PiecePlacement(7, 0);

            for char in self.0.trim().chars() {
                match &mut section {
                    PiecePlacement(rank, file) => {
                        if char.is_numeric() {
                            let number = char.to_digit(10).unwrap() as i8;
                            if number + *file > 8 {
                                return Err(FenError(format!("Rank {rank} exceeds 8 files")));
                            }

                            *file += number;

                            continue;
                        }

                        if char.is_alphabetic() {
                            if *file == 8 {
                                return Err(FenError(format!("Rank {rank} exceeds 8 files")));
                            }

                            let tile = Tile::from((*rank * 8 + *file) as u8);

                            match char {
                                'K' => {
                                    if board.pieces[WHITE_KING].is_some() {
                                        return Err(FenError("White King already exists".into()));
                                    }

                                    board.set_tile(tile, WHITE_KING);
                                }
                                'Q' => board.set_tile(tile, WHITE_QUEEN),
                                'R' => board.set_tile(tile, WHITE_ROOK),
                                'B' => board.set_tile(tile, WHITE_BISHOP),
                                'N' => board.set_tile(tile, WHITE_KNIGHT),
                                'P' => board.set_tile(tile, WHITE_PAWN),
                                'k' => {
                                    if board.pieces[BLACK_KING].is_some() {
                                        return Err(FenError("Black King already exists".into()));
                                    }

                                    board.set_tile(tile, BLACK_KING);
                                }
                                'q' => board.set_tile(tile, BLACK_QUEEN),
                                'r' => board.set_tile(tile, BLACK_ROOK),
                                'b' => board.set_tile(tile, BLACK_BISHOP),
                                'n' => board.set_tile(tile, BLACK_KNIGHT),
                                'p' => board.set_tile(tile, BLACK_PAWN),
                                _ => {
                                    return Err(FenError(format!(
                                        "Invalid piece character {char}"
                                    )));
                                }
                            }

                            *file += 1;

                            continue;
                        }

                        if char == '/' {
                            if *file != 8 {
                                return Err(FenError(format!(
                                    "Rank {rank} does not contain 8 files"
                                )));
                            }

                            *rank -= 1;
                            *file = 0;

                            continue;
                        }

                        if char == ' ' {
                            if *rank != -1 && *file != 8 {
                                return Err(FenError(
                                    "Not all ranks and files have been filled up".into(),
                                ));
                            }

                            section = ActiveColor(false);

                            continue;
                        }

                        return Err(FenError("Invalid piece placement character".into()));
                    }
                    ActiveColor(state) => {
                        match char {
                            'w' => board.turn = PieceColor::White,
                            'b' => board.turn = PieceColor::Black,
                            ' ' => {
                                if !*state {
                                    return Err(FenError("Unset active color".into()));
                                }

                                section = CastlingRights(false);

                                continue;
                            }
                            _ => return Err(FenError("Invalid active color character".into())),
                        }

                        section = ActiveColor(true)
                    }
                    CastlingRights(state) => {
                        let castling = u8::from(&board.castling);
                        match char {
                            'K' => board.castling |= CastlingDirection::WhiteKing,
                            'Q' => board.castling |= CastlingDirection::WhiteQueen,
                            'k' => board.castling |= CastlingDirection::BlackKing,
                            'q' => board.castling |= CastlingDirection::BlackQueen,
                            '-' => {
                                if castling != 0 {
                                    return Err(FenError("Invalid castling rights string".into()));
                                }
                            }
                            ' ' => {
                                if !*state {
                                    return Err(FenError("Unset castling rights".into()));
                                }

                                section = PossibleEnPassantTargets("".into());

                                continue;
                            }
                            _ => return Err(FenError("Invalid castling rights character".into())),
                        }

                        section = CastlingRights(true)
                    }
                    PossibleEnPassantTargets(string) => {
                        if char != ' ' {
                            section = PossibleEnPassantTargets(format!("{string}{char}"));

                            continue;
                        }

                        if string.is_empty() {
                            return Err(FenError(
                                "Invalid possible en passant targets string".into(),
                            ));
                        }

                        if string == "-" {
                            section = HalfMoveClock("".into());

                            continue;
                        }

                        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
                        let ranks = ['1', '2', '3', '4', '5', '6', '7', '8'];

                        let mut chars = string.chars();
                        let file = chars.next().unwrap();
                        let rank = chars.next().unwrap();

                        if string.len() == 2 && files.contains(&file) && ranks.contains(&rank) {
                            let rank = rank.to_digit(10).unwrap() as u8;
                            let file = files.iter().position(|r| *r == file).unwrap() as u8;

                            board.enpassant = Bitboard::from(((rank - 1) * 8) + file);

                            section = HalfMoveClock("".into());
                        } else {
                            return Err(FenError(
                                "Invalid possible en passant targets string".into(),
                            ));
                        }
                    }
                    HalfMoveClock(string) => {
                        if char != ' ' {
                            section = HalfMoveClock(format!("{string}{char}"));

                            continue;
                        }

                        if string.is_empty() {
                            return Err(FenError("Invalid half move clock string".into()));
                        }

                        match string.parse::<u8>() {
                            Ok(number) => {
                                board.halfmove = number;

                                section = FullMoveNumber("".into());

                                continue;
                            }
                            Err(_) => {
                                return Err(FenError("Invalid half move clock string".into()));
                            }
                        }
                    }
                    FullMoveNumber(string) => {
                        if char != ' ' {
                            section = FullMoveNumber(format!("{string}{char}"));

                            continue;
                        }

                        if string.is_empty() {
                            return Err(FenError("Invalid full move number string".into()));
                        }

                        match string.parse::<u8>() {
                            Ok(number) => {
                                board.fullmove = number;

                                section = Finished;

                                continue;
                            }
                            Err(_) => {
                                return Err(FenError("Invalid full move number string".into()));
                            }
                        }
                    }
                    Finished => {
                        return Err(FenError("Invalid characters at the end".into()));
                    }
                }
            }

            for color in [PieceColor::White, PieceColor::Black] {
                board.update_rays(color);
                board.update_attacks(color);
                board.update_pin_lines(color);
                board.update_check_count(color);
            }

            Ok(board)
        })
    }
}
