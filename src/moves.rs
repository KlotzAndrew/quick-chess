use crate::board::{Bitboard, Board};

pub fn best_move(board: Board) -> String {
    let moves = pawn_moves(board);
    let best_move = moves[0];
    let move_string = move_string(best_move);

    move_string
}

pub fn apply_move(board: Board, last_move: &str) -> Board {
    board
}

#[derive(Clone, Copy)]
pub struct Move {
    pub from: u64,
    pub to: u64,
}

fn pawn_moves(board: Board) -> Vec<Move> {
    let pawns = board.pawns() & board.white();
    let max = Bitboard(u64::MAX);
    let empty_squares = (board.black() | board.white()) ^ max;

    let moves = single_pushes(pawns.clone(), empty_squares);

    moves
}

fn single_pushes(mut pawns: Bitboard, empty_squares: Bitboard) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    while pawns != Bitboard(0) {
        let index = pawns.0.trailing_zeros();
        let source_square = 1 << index;

        pawns = pawns.clear_bit(index);

        let target_square = source_square << 8;

        if !empty_squares.is_bit_set(target_square as u16) {
            let m = Move {
                to: target_square,
                from: source_square,
            };

            moves.push(m);
        }
    }
    moves
}

pub fn bitboard_to_square(bitboard: Bitboard) -> u32 {
    let bits = bitboard.0.trailing_zeros() + 1;
    return bits / 8 + bits % 8;
}

pub fn rank_of(square: u32) -> u32 {
    square / 8
}

pub fn file_of(square: u32) -> u32 {
    square % 8
}

pub fn rank_of_bitboard(bitboard: Bitboard) -> u32 {
    let zeros = bitboard.0.trailing_zeros();
    zeros / 8
}

pub fn file_of_bitboard(bitboard: Bitboard) -> u32 {
    bitboard.0.trailing_zeros() % 8
}

pub fn file_to_char(file: u32) -> char {
    match file {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => panic!("unknown file {}", file),
    }
}

pub fn move_string(m: Move) -> String {
    let from_square = bitboard_to_square(Bitboard(m.from));
    let to_square = bitboard_to_square(Bitboard(m.to));

    format!(
        "{}{}{}{}",
        file_to_char(file_of_bitboard(Bitboard(m.from))),
        rank_of_bitboard(Bitboard(m.from)) + 1,
        file_to_char(file_of_bitboard(Bitboard(m.to))),
        rank_of_bitboard(Bitboard(m.to)) + 1
    )
}

// const FileValues: &'static [u64] = [Square::A1 | Square::B1];
// 1_0000_0000

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_string() {
        // let square = bitboard_to_square(Bitboard(1 << 8));
        let bb = Bitboard(1);

        assert_eq!(
            "a2a3",
            move_string(Move {
                from: 1 << 8,
                to: 1 << 16
            })
        )
    }

    #[test]
    fn test_pawn_move() {
        let current_board = Board::new();

        let moves = pawn_moves(current_board);

        assert_eq!(moves.len(), 8);

        let first_move = moves[0];
        for m in moves.iter() {
            println!("{:0b} {:0b}", m.from, m.to);
            println!("{}", move_string(m.clone()));
        }

        println!(
            "zero {:0b} {}",
            Bitboard(1),
            move_string(Move {
                from: 1,
                to: 1 << 8
            })
        );

        assert_eq!("a2a3", move_string(first_move));
    }
}
