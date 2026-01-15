use crate::core::game::{Board, Piece};
use std::io::{self, BufRead};

/// Stateful turn reader. It keeps a single `Lines` iterator
/// across frames — reading Anfield + Piece per turn.
pub struct TurnReader {
    lines: std::io::Lines<std::io::StdinLock<'static>>,
}

impl TurnReader {
    pub fn new() -> Self {
        // "Leak" stdin lock to 'static. Practical here; program is short-lived.
        let stdin: &'static io::Stdin = Box::leak(Box::new(io::stdin()));
        let lines = stdin.lock().lines();
        Self { lines }
    }

    pub fn read_turn(&mut self) -> Option<(Board, Piece)> {
        // Find "Anfield W H:"
        let (w, h) = loop {
            let l = self.lines.next()?.ok()?;
            if let Some((w, h)) = parse_dims(&l, "Anfield") {
                break (w, h);
            }
        };

        // One header line with indices, ignore it.
        let _ = self.lines.next();

        let mut grid = Vec::with_capacity(h);
        for _ in 0..h {
            let line = self.lines.next()?.ok()?;
            grid.push(extract_row(&line).chars().collect());
        }
        let board = Board { h, w, cells: grid };

        // Now read piece header: "Piece w h:"
        let hdr = self.lines.next()?.ok()?;
        let (pw, ph) = parse_dims(&hdr, "Piece")?;
        let mut pc = Vec::with_capacity(ph);
        for _ in 0..ph {
            let l = self.lines.next()?.ok()?;
            pc.push(l.chars().collect());
        }
        let piece = Piece { h: ph, w: pw, cells: pc };

        Some((board, piece))
    }
}

fn parse_dims(line: &str, tag: &str) -> Option<(usize, usize)> {
    if !line.starts_with(tag) { return None; }
    // Example: "Anfield 20 15:" or "Piece 4 1:"
    let mut it = line.split_whitespace().skip(1);
    let a = it.next()?.parse::<usize>().ok()?;
    let b = it.next()?.trim_end_matches(':').parse::<usize>().ok()?;
    // For Anfield: a = W, b = H; for Piece: same convention.
    Some((a, b))
}

fn extract_row(line: &str) -> &str {
    // Lines look like "003 .......$......." → we need the trailing glyphs
    line.split_whitespace().last().unwrap_or("")
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_dims_works_for_anfield_and_piece() {
        assert_eq!(super::parse_dims("Anfield 40 30:", "Anfield"), Some((40, 30)));
        assert_eq!(super::parse_dims("Piece 5 3:", "Piece"), Some((5, 3)));
    }

    #[test]
    fn parse_dims_rejects_other_lines() {
        assert_eq!(super::parse_dims("whatever 10 2:", "Anfield"), None);
        assert_eq!(super::parse_dims("Piece X 3:", "Piece"), None);
        assert_eq!(super::parse_dims("Piece 3 X:", "Piece"), None);
    }

    #[test]
    fn extract_row_takes_last_whitespace_chunk() {
        assert_eq!(super::extract_row("003 .......$......."), ".......$.......");
        assert_eq!(super::extract_row("010 @@.@@"), "@@.@@");
        assert_eq!(super::extract_row("bogus"), "bogus");
        assert_eq!(super::extract_row(""), "");
    }
}



