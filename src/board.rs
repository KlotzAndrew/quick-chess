use std::fmt::{self, Write};

use std::ops::BitAnd;
use std::ops::BitXor;

#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    fn char(self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }

    fn char_upper(self) -> char {
        match self {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }
}

#[derive(PartialEq)]
pub enum Color {
    Black = 0,
    White = 1,
}

impl Color {
    fn from_white(white: bool) -> Color {
        if white {
            Color::White
        } else {
            Color::Black
        }
    }
}

pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    fn is_white(&self) -> bool {
        self.color == Color::White
    }
    fn char(&self) -> char {
        if self.is_white() {
            self.piece_type.char()
        } else {
            self.piece_type.char_upper()
        }
    }
}

impl Piece {}

#[derive(Copy, Clone)]
pub struct BoardColor {
    pub white: Bitboard,
    pub black: Bitboard,
}

#[derive(Copy, Clone)]
pub struct Board {
    colors: [Bitboard; 2],
    pieces: [Bitboard; 6],
}

impl Board {
    pub fn new() -> Board {
        Board {
            colors: [
                Bitboard::new(0b1111111111111111000000000000000000000000000000000000000000000000),
                Bitboard::new(0b0000000000000000000000000000000000000000000000001111111111111111),
            ],
            pieces: [
                Bitboard::new(0b0000000011111111000000000000000000000000000000001111111100000000), // pawns
                Bitboard::new(0b0100001000000000000000000000000000000000000000000000000001000010), // knights
                Bitboard::new(0b0010010000000000000000000000000000000000000000000000000000100100), // bishops
                Bitboard::new(0b1000000100000000000000000000000000000000000000000000000010000001), // rooks
                Bitboard::new(0b0000100000000000000000000000000000000000000000000000000000001000), //queens
                Bitboard::new(0b0001000000000000000000000000000000000000000000000000000000010000), // kings
            ],
        }
    }

    pub fn print(self) {
        println!("{:?}", &self)
    }

    fn pawns(&self) -> Bitboard {
        self.pieces[0]
    }

    fn knights(&self) -> Bitboard {
        self.pieces[1]
    }

    fn bishops(&self) -> Bitboard {
        self.pieces[2]
    }

    fn rooks(&self) -> Bitboard {
        self.pieces[3]
    }

    fn queens(&self) -> Bitboard {
        self.pieces[4]
    }

    fn kings(&self) -> Bitboard {
        self.pieces[5]
    }

    fn black(&self) -> Bitboard {
        self.colors[0]
    }

    fn white(&self) -> Bitboard {
        self.colors[1]
    }

    fn piece_type_at(self, row: u16, col: u16) -> Option<PieceType> {
        let square = coords_to_bit(row, col);
        if self.pawns().is_bit_set(square) {
            Some(PieceType::Pawn)
        } else if self.knights().is_bit_set(square) {
            Some(PieceType::Knight)
        } else if self.bishops().is_bit_set(square) {
            Some(PieceType::Bishop)
        } else if self.rooks().is_bit_set(square) {
            Some(PieceType::Rook)
        } else if self.queens().is_bit_set(square) {
            Some(PieceType::Queen)
        } else if self.kings().is_bit_set(square) {
            Some(PieceType::King)
        } else {
            None
        }
    }

    fn piece_at(self, row: u16, col: u16) -> Option<Piece> {
        let square = coords_to_bit(row, col);
        let piece_type = self.piece_type_at(row, col);

        piece_type.map(|piece_type| Piece {
            color: Color::from_white(self.white().is_bit_set(square)),
            piece_type,
        })
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                let piece = self.piece_at(row, col);

                let c = piece.map_or('.', |piece| piece.char());

                fmt.write_char(c)?;

                fmt.write_char(' ')?;
            }
            fmt.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq)]

pub struct Bitboard(u64);

impl Bitboard {
    pub fn new(n: u64) -> Bitboard {
        Bitboard(n)
    }

    pub fn print(self) {
        println!("{:?}", &self)
    }

    fn is_bit_set(self, i: u16) -> bool {
        (self & (Bitboard::new(1 << i))) != Bitboard::new(0)
    }
}

// needed to format! as binary
impl fmt::Binary for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

// needed for print {:?}
impl fmt::Debug for Bitboard {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                if self.is_bit_set(coords_to_bit(row, col)) {
                    fmt.write_char('1')?;
                } else {
                    fmt.write_char('.')?;
                }
                fmt.write_char(' ')?;
            }
            fmt.write_char('\n')?;
        }

        Ok(())
    }
}

// needed for & operation
impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Bitboard) -> Bitboard {
        Self(self.0 ^ rhs.0)
    }
}

fn coords_to_bit(row: u16, col: u16) -> u16 {
    row * 8 + col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_board() {
        let current_board = Board::new();
        current_board.print();

        // assert_eq!(1, 1);

        let white_and_black = current_board.white() ^ current_board.black();

        let all_pieces = current_board.pawns()
            ^ current_board.knights()
            ^ current_board.bishops()
            ^ current_board.rooks()
            ^ current_board.queens()
            ^ current_board.kings();

        assert_eq!(
            format!("{:0b}", white_and_black),
            format!("{:0b}", all_pieces)
        );

        let zero_pieces = current_board.pawns()
            & current_board.knights()
            & current_board.bishops()
            & current_board.rooks()
            & current_board.queens()
            & current_board.kings();

        assert_eq!(format!("{:0b}", zero_pieces), format!("{:0b}", 0));
        assert_eq!(
            format!("{:0b}", current_board.white() & current_board.black()),
            format!("{:0b}", 0)
        );
    }

    #[test]
    fn test_print_bitboard() {
        let current_bb =
            Bitboard::new(0b0000000000000000000000000000000000000000000000000000000010000001);

        current_bb.print();

        assert_eq!(format!("{:0b}", current_bb), format!("{:0b}", 129))
    }
}
