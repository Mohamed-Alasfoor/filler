mod core;
mod io;
mod rules;
mod strategy;

use crate::core::game::Symbols;
use io::reader::TurnReader;
use io::whoami::detect_player;
use strategy::greedy::choose_move;

use std::io::{self as sysio, Write}; 

fn main() {
    //to discover who I am 
    let symbols: Symbols = detect_player();

    // Keep one reader that streams frames; stdin
    let mut turn_reader = TurnReader::new();

    // Buffered stdout to play nice with the engine.
    let mut out = sysio::BufWriter::new(sysio::stdout());


    // Main loop: read frame -> pick move -> print "X Y"
    while let Some((board, piece)) = turn_reader.read_turn() {
        let (x, y) = choose_move(&board, &piece, &symbols).unwrap_or((0, 0));
        let _ = writeln!(out, "{} {}", x, y);
        let _ = out.flush();
    }
}
