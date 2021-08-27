use std::fmt::{self, Write};

use std::ops::BitAnd;

enum PieceIndex {
    Pawns = 0,
    Knights = 1,
    Bishops = 2,
    Rooks = 3,
    Queens = 4,
    Kings = 5,
}

#[derive(Copy, Clone)]
pub struct Board {
    black_pieces: [Bitboard; 6],
    white_pieces: [Bitboard; 6],
}

impl Board {
    pub fn new() -> Board {
        Board {
            black_pieces: [
                Bitboard::new(0), // pawns
                Bitboard::new(0), // knight
                Bitboard::new(0), // bishops
                Bitboard::new(0b1000000100000000000000000000000000000000000000000000000000000000), // rooks
                Bitboard::new(0), // queens
                Bitboard::new(0), // kings
            ],
            white_pieces: [
                Bitboard::new(0),
                Bitboard::new(0),
                Bitboard::new(0),
                Bitboard::new(0b0000000000000000000000000000000000000000000000000000000010000001),
                Bitboard::new(0),
                Bitboard::new(0),
            ],
        }
    }

    pub fn print(self) {
        println!("{:?}", &self)
    }

    fn piece_at(self, row: u16, col: u16) -> Option<String> {
        let square = coords_to_bit(row, col);
        if self.black_pieces[3].is_bit_set(square) {
            Some("R".to_owned())
        } else if self.white_pieces[3].is_bit_set(square) {
            Some("r".to_owned())
        } else {
            None
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                let piece = self.piece_at(row, col);

                let s = piece.unwrap_or(".".to_owned());

                fmt.write_str(&s)?;
                // if self.is_bit_set(coords_to_bit(row, col)) {
                //     fmt.write_char('1')?;
                // } else {
                //     fmt.write_char('.')?;
                // }
                fmt.write_char(' ')?;
            }
            fmt.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq)]

struct Bitboard(u64);

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

        assert_eq!(1, 1)
    }

    #[test]
    fn test_print_bitboard() {
        let current_bb =
            Bitboard::new(0b0000000000000000000000000000000000000000000000000000000010000001);

        current_bb.print();

        assert_eq!(format!("{:0b}", current_bb), format!("{:0b}", 129))
    }
}
