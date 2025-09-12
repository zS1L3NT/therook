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

impl From<&Board> for String {
    fn from(board: &Board) -> Self {
        let state = board.get_state();

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

            if state.castling[WHITE_KING] {
                fen.push('K');
            }

            if state.castling[WHITE_QUEEN] {
                fen.push('Q');
            }

            if state.castling[BLACK_KING] {
                fen.push('k');
            }

            if state.castling[BLACK_QUEEN] {
                fen.push('q');
            }

            if state.castling == [false; 4] {
                fen.push('-');
            }

            fen.push(' ');

            if state.enpassant.is_some() {
                let square = state.enpassant.into_iter().find_or_first(|_| true).unwrap();
                fen.push_str(&format!(
                    "{}{}",
                    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][square as usize & 7],
                    ['1', '2', '3', '4', '5', '6', '7', '8'][square as usize >> 3]
                ))
            } else {
                fen.push('-');
            }

            fen.push(' ');

            fen.push_str(&format!("{}", state.halfmove));

            fen.push(' ');

            fen.push_str(&format!("{}", state.fullmove));

            fen
        })
    }
}

impl TryFrom<&str> for Board {
    type Error = String;

    fn try_from(fen: &str) -> Result<Board, Self::Error> {
        timed!("parsing fen into board", {
            let mut board = Board::new();
            let mut state = BoardState::new();
            let mut section = PiecePlacement(7, 0);

            for char in fen.trim().chars() {
                match &mut section {
                    PiecePlacement(rank, file) => {
                        if char.is_numeric() {
                            let number = char.to_digit(10).unwrap() as i8;
                            if number + *file > 8 {
                                return Err(format!(
                                    "Invalid piece placement: Rank {rank} exceeds 8 files"
                                ));
                            }

                            *file += number;

                            continue;
                        }

                        if char.is_alphabetic() {
                            if *file == 8 {
                                return Err(format!(
                                    "Invalid piece placement: Rank {rank} exceeds 8 files"
                                ));
                            }

                            let square = (*rank * 8 + *file) as u8;

                            match char {
                                'K' => {
                                    if board.pieces[WHITE_KING].is_some() {
                                        return Err(
                                            "Invalid piece placement: White King already exists"
                                                .into(),
                                        );
                                    }

                                    board.set_square(square, WHITE_KING);
                                }
                                'Q' => board.set_square(square, WHITE_QUEEN),
                                'R' => board.set_square(square, WHITE_ROOK),
                                'B' => board.set_square(square, WHITE_BISHOP),
                                'N' => board.set_square(square, WHITE_KNIGHT),
                                'P' => board.set_square(square, WHITE_PAWN),
                                'k' => {
                                    if board.pieces[BLACK_KING].is_some() {
                                        return Err(
                                            "Invalid piece placement: Black King already exists"
                                                .into(),
                                        );
                                    }

                                    board.set_square(square, BLACK_KING);
                                }
                                'q' => board.set_square(square, BLACK_QUEEN),
                                'r' => board.set_square(square, BLACK_ROOK),
                                'b' => board.set_square(square, BLACK_BISHOP),
                                'n' => board.set_square(square, BLACK_KNIGHT),
                                'p' => board.set_square(square, BLACK_PAWN),
                                _ => {
                                    return Err(format!(
                                        "Invalid piece placement: Unknown character {char}"
                                    ));
                                }
                            }

                            *file += 1;

                            continue;
                        }

                        if char == '/' {
                            if *file != 8 {
                                return Err(format!(
                                    "Invalid piece placement: Rank {rank} does not contain 8 files"
                                ));
                            }

                            *rank -= 1;
                            *file = 0;

                            continue;
                        }

                        if char == ' ' {
                            if *rank != -1 && *file != 8 {
                                return Err("Invalid piece placement: Not all ranks and files have been filled up".into());
                            }

                            section = ActiveColor(false);

                            continue;
                        }

                        return Err(format!("Invalid piece placement: Unknown character {char}"));
                    }
                    ActiveColor(is_set) => {
                        match char {
                            'w' => board.turn = PieceColor::White,
                            'b' => board.turn = PieceColor::Black,
                            ' ' => {
                                if !*is_set {
                                    return Err("Unset active color".into());
                                }

                                section = CastlingRights(false);

                                continue;
                            }
                            _ => {
                                return Err(format!(
                                    "Invalid active color: Unknown character {char}"
                                ));
                            }
                        }

                        section = ActiveColor(true)
                    }
                    CastlingRights(is_set) => {
                        match char {
                            'K' => state.castling[WHITE_KING] = true,
                            'Q' => state.castling[WHITE_QUEEN] = true,
                            'k' => state.castling[BLACK_KING] = true,
                            'q' => state.castling[BLACK_QUEEN] = true,
                            '-' => {
                                if state.castling != [false; 4] {
                                    return Err("Invalid castling rights: Cannot use - when setting other castling rights".into());
                                }
                            }
                            ' ' => {
                                if !*is_set {
                                    return Err(
                                        "Invalid castling rights: No castling rights provided"
                                            .into(),
                                    );
                                }

                                section = PossibleEnPassantTargets("".into());

                                continue;
                            }
                            _ => {
                                return Err(format!(
                                    "Invalid castling rights: Unknown character {char}"
                                ));
                            }
                        }

                        section = CastlingRights(true)
                    }
                    PossibleEnPassantTargets(string) => {
                        if char != ' ' {
                            section = PossibleEnPassantTargets(format!("{string}{char}"));

                            continue;
                        }

                        if string.is_empty() {
                            return Err("Invalid possible en passant targets: No possible en passant target provided".into());
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

                            state.enpassant = Bitboard::from(((rank - 1) * 8) + file);

                            section = HalfMoveClock("".into());
                        } else {
                            return Err(format!(
                                "Invalid possible en passant targets: Unknown square {string}",
                            ));
                        }
                    }
                    HalfMoveClock(string) => {
                        if char != ' ' {
                            section = HalfMoveClock(format!("{string}{char}"));

                            continue;
                        }

                        if string.is_empty() {
                            return Err(
                                "Invalid half move clock: No half move clock provided".into()
                            );
                        }

                        match string.parse::<u8>() {
                            Ok(number) => {
                                state.halfmove = number;

                                section = FullMoveNumber("".into());

                                continue;
                            }
                            Err(_) => {
                                return Err(format!(
                                    "Invalid half move clock: Invalid number {string}"
                                ));
                            }
                        }
                    }
                    FullMoveNumber(string) => {
                        if char != ' ' {
                            section = FullMoveNumber(format!("{string}{char}"));

                            continue;
                        }

                        if string.is_empty() {
                            return Err(
                                "Invalid full move number: No full move number provided".into()
                            );
                        }

                        match string.parse::<u8>() {
                            Ok(number) => {
                                state.fullmove = number;

                                section = Finished;

                                continue;
                            }
                            Err(_) => {
                                return Err(format!(
                                    "Invalid full move number: Invalid number {string}"
                                ));
                            }
                        }
                    }
                    Finished => {
                        return Err(
                            "Invalid FEN: Extra characters provided at the end of the string"
                                .into(),
                        );
                    }
                }
            }

            board.states.push(state);

            for color in [PieceColor::White, PieceColor::Black] {
                board.update_rays(color);
                board.update_attacks(color);
                board.update_pin_lines(color);
            }

            Ok(board)
        })
    }
}

impl PartialEq<str> for Board {
    fn eq(&self, other: &str) -> bool {
        String::from(self) == String::from(other)
    }
}
