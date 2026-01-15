use crate::core::game::Symbols;
use std::io::{self, BufRead};

/// Reads lines until it finds "$$$ exec p1|p2 : ...".
pub fn detect_player() -> Symbols {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(Ok(l)) = lines.next() {
        if l.starts_with("$$$") && l.contains("exec") {
            if l.contains("p1") { return Symbols::p1(); }
            if l.contains("p2") { return Symbols::p2(); }
        }
        // Eat noise until the engine settles.
    }
    Symbols::p1()
}

