use log::{debug, error, info, trace, warn};
use std::io::{self, BufRead, Read};

mod logs;

fn main() {
    logs::setup().expect("unable to configure logger");

    let mut board = BoardModel::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,
            Ok(line) => board = handle_line(board, line),
        }
    }
}

fn handle_line(board: BoardModel, line: String) -> BoardModel {
    let command: Vec<&str> = line.split_whitespace().collect();

    if line == "uci" {
        write_msg("id name quick-chess");
        write_msg("id author Andrew Klotz");
        write_msg("uciok");
    } else if line == "isready" {
        write_msg("readyok")
    } else if line == "ucinewgame" {
        // game started with empty board
    } else if command[0] == "position" {
        // position startpos
        // position startpos moves e2e3
        if command.len() == 2 {
            // first move
            write_msg("bestmove e2e3")
        } else {
            let last_move = command[3];
            let (next_board, next_move) = find_next_move(board, last_move);
            return next_board;
        }
        // position startpos moves e2e4
    } else if command[0] == "go" {
        // go wtime 2000 btime 2000 movestogo 180
        // time control info comes after last move
    } else if line == "stop" {
        // stop searching, is there a resume option?
    } else if line == "quit" {
        // exit the program
    } else {
        write_msg(&format!("unhandled msg: {}", line));
    }
    board
}

fn find_next_move(board: BoardModel, last_move: &str) -> (BoardModel, &str) {
    (board, "e2e3")
}

#[derive(Copy, Clone)]
struct BoardModel {}

impl BoardModel {
    pub fn new() -> BoardModel {
        BoardModel {}
    }
}

fn write_msg(message: &str) {
    print!("{}\n", message);
    info!("{}", message);
}
