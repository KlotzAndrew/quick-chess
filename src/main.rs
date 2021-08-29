use log::info;
use std::io::{self, BufRead};

mod board;
mod logs;
mod moves;

fn main() {
    logs::setup().expect("unable to configure logger");

    let mut board = board::Board::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,
            Ok(line) => board = handle_line(board, line),
        }
    }
}

fn handle_line(mut board: board::Board, line: String) -> board::Board {
    println!("recieved ---- {}", line);
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
            // write_msg("bestmove e2e4")
        } else {
            // apply opposition player move
            let last_move = command[3];
            board = moves::apply_move(board, last_move);
        }
        let next_move = moves::best_move(board);
        write_msg(&format!("bestmove {}", next_move));
        return moves::apply_move(board, &next_move);
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

fn write_msg(message: &str) {
    print!("{}\n", message);
    info!("{}", message);
}
