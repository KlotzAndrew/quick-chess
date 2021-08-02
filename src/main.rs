use log::{debug, error, info, trace, warn};
use std::io::{self, BufRead, Read};

mod logs;

fn main() {
    logs::setup().expect("unable to configure logger");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break,
            Ok(line) => handle_line(line),
        }
    }
}

fn handle_line(line: String) {
    if line == "uci" {
        write_msg("id name quick-chess");
        write_msg("id author Andrew Klotz");
        write_msg("uciok");
    } else if line == "isready" {
        write_msg("readyok")
    } else if line == "ucinewgame" {
        // game started with empty board
    } else if line == "position startpos" {
        // position startpos moves e2e4
        write_msg("bestmove e2e4")
    } else if line == "stop" {
        // stop searching, is there a resume option?
    } else if line == "quit" {
        // exit the program
    } else {
        write_msg(&format!("unhandled msg: {}", line));
    }
}

fn write_msg(message: &str) {
    print!("{}\n", message);
    info!("{}", message);
}
