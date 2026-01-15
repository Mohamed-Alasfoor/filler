use crate::core::game::{Board, Piece, Symbols};
use crate::rules::validator::legal;

/// Collect enemy cells as (y, x). Kept simple & local for cache
fn enemy_cells(board: &Board, sym: &Symbols) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for y in 0..board.h {
        for x in 0..board.w {
            let c = board.cells[y][x];
            if sym.is_op(c) { out.push((y as i32, x as i32)); }
        }
    }
    out
}

/// Basic distance 
#[inline]
fn manhattan(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

/// Score a placement by its distance to the nearest enemy cell.
fn placement_score(_board: &Board, piece: &Piece, y: i32, x: i32, foes: &[(i32, i32)]) -> (i32, i32, i32) {
    let mut best = i32::MAX;

    if foes.is_empty() {
        // Early turns: just hug top-left to reduce variance.
        return (0, y, x);
    }

    for py in 0..piece.h {
        for px in 0..piece.w {
            if piece.cells[py][px] == '.' { continue; }
            let gy = y + py as i32;
            let gx = x + px as i32;

            for &e in foes {
                let d = manhattan((gy, gx), e);
                if d < best { best = d; }
            }
        }
    }
    (best, y, x)
}

/// Enumerate over a safe window of top-lefts. We allow negative offsets so the
/// piece can start outside as long as its filled cells land in-bounds.
fn iter_top_lefts(board: &Board, piece: &Piece) -> impl Iterator<Item = (i32, i32)> {
    let y_min = -(piece.h as i32) + 1;
    let x_min = -(piece.w as i32) + 1;
    let y_max = board.h as i32 - 1;
    let x_max = board.w as i32 - 1;

    (y_min..=y_max).flat_map(move |y| (x_min..=x_max).map(move |x| (y, x)))
}

/// Returns (x, y) for printing, or None if no legal moves.
pub fn choose_move(board: &Board, piece: &Piece, sym: &Symbols) -> Option<(i32, i32)> {
    let foes = enemy_cells(board, sym);

    let mut best: Option<(i32, i32, i32)> = None; // (score, y, x)

    for (y, x) in iter_top_lefts(board, piece) {
        if !legal(board, piece, y, x, sym) { continue; }
        let s = placement_score(board, piece, y, x, &foes);

        // min-heap by tuple ordering (score, y, x)
        best = match best {
            None => Some(s),
            Some(curr) => Some(if s < curr { s } else { curr }),
        };
    }

    best.map(|(_, y, x)| (x, y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::game::{Board, Piece};

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
    fn manhattan_distance_is_correct() {
        assert_eq!(super::manhattan((0,0),(0,0)), 0);
        assert_eq!(super::manhattan((0,0),(3,4)), 7);
        assert_eq!(super::manhattan((-2,5),(1,1)), 7);
    }

    #[test]
    fn no_foes_hugs_top_left_among_legal_moves() {
        // single my cell at (2,2); 1x1 piece must overlap exactly one → only (2,2) is legal
        let b = board_from(&[
            ".....",
            ".....",
            "..@..",
            ".....",
            ".....",
        ]);
        let p = piece_from(&["O"]);
        let mv = choose_move(&b, &p, &Symbols::p1()).expect("move");
        assert_eq!(mv, (2, 2));
    }

    #[test]
    fn tie_breaker_prefers_smaller_y_then_x() {
        // symmetric enemies; two legal placements with same score → choose lower y, then lower x
        let b = board_from(&[
            "@....",
            ".....",
            "....@",
        ]);
        let p = piece_from(&["O"]); // must land exactly on one my cell → (0,0) or (2,4)
        // both are legal; distances to the other '@' are equal
        let mv = choose_move(&b, &p, &Symbols::p1()).expect("move");
        assert_eq!(mv, (0, 0)); // smaller y wins
    }

    #[test]
    fn returns_none_when_no_legal_moves() {
        // board with no my cells → no legal moves (overlap must be exactly one)
        let b = board_from(&[".....", "....."]);
        let p = piece_from(&["O"]);
        assert!(choose_move(&b, &p, &Symbols::p1()).is_none());
    }

}


