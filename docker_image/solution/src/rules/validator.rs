use crate::core::game::{Board, Piece, Symbols};

/// Check if placing `piece` with top-left at (y, x) is legal:
/// - every filled cell must be in bounds
/// - cannot overlap opponent
/// - overlaps *exactly one* of our cells
pub fn legal(board: &Board, piece: &Piece, y: i32, x: i32, sym: &Symbols) -> bool {
    let mut overlap = 0usize;

    for py in 0..piece.h {
        for px in 0..piece.w {
            let pc = piece.cells[py][px];
            if pc == '.' { continue; }

            let ay = y + py as i32;
            let ax = x + px as i32;

            if ay < 0 || ax < 0 || ay >= board.h as i32 || ax >= board.w as i32 {
                return false;
            }

            let bc = board.cells[ay as usize][ax as usize];
            if sym.is_op(bc) {
                return false;
            }
            if bc != '.' {
                overlap += 1;
                if overlap > 1 { return false; }
            }
        }
    }
    overlap == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    fn board_from(rows: &[&str]) -> Board {
        let h = rows.len();
        let w = rows.first().map(|r| r.len()).unwrap_or(0);
        Board { h, w, cells: rows.iter().map(|r| r.chars().collect()).collect() }
    }
    fn piece_from(rows: &[&str]) -> Piece {
        let h = rows.len();
        let w = rows.first().map(|r| r.len()).unwrap_or(0);
        Piece { h, w, cells: rows.iter().map(|r| r.chars().collect()).collect() }
    }

    #[test]
    fn rejects_empty_piece_no_filled_cells() {
        let b = board_from(&["....", ".@.."]);
        let p = piece_from(&["..", ".."]);
        assert!(!legal(&b, &p, 0, 0, &Symbols::p1()));
    }


    #[test]
    fn enemy_upper_and_lowercase_are_forbidden() {
        let b = board_from(&[".$s.", "...."]);
        let p = piece_from(&["OO"]);
        assert!(!legal(&b, &p, 0, 0, &Symbols::p1())); // hits '$'
        assert!(!legal(&b, &p, 0, 1, &Symbols::p1())); // hits 's'
    }

    #[test]
    fn out_of_bounds_any_filled_cell_is_illegal() {
        let b = board_from(&[
            "@...",
            "....",
        ]);
        let p = piece_from(&["OO"]);
        // left O is out of bounds (x=-1) → illegal even if the other overlaps
        assert!(!legal(&b, &p, 0, -1, &Symbols::p1()));
        // bottom O is out (y=2) → illegal
        assert!(!legal(&b, &p, 1, 0, &Symbols::p1()));
    }

    #[test]
    fn negative_origin_ok_if_only_dots_outside() {
        // our cell at (0,0)
        let b = board_from(&["@..", "..."]);
        // piece with leading '.' then 'O'
        let p = piece_from(&[".O"]);
        // top-left (0,-1): the '.' is outside (ignored), 'O' lands at (0,0) → exactly one overlap → legal
        assert!(legal(&b, &p, 0, -1, &Symbols::p1()));
    }

    #[test]
    fn complex_piece_checks_each_filled_cell() {
        let b = board_from(&[
            ".@..",
            "....",
            "..$.",
            "....",
        ]);
        let p = piece_from(&[
            "OO.",
            ".O.",
        ]);
        // place to touch my '@' once and no enemy
        assert!(legal(&b, &p, 0, 0, &Symbols::p1()));
        // shift so bottom right would hit '$' → illegal
        assert!(!legal(&b, &p, 1, 1, &Symbols::p1()));
    }
}

