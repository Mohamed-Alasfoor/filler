#[derive(Clone)]
pub struct Board {
    pub h: usize,
    pub w: usize,
    pub cells: Vec<Vec<char>>,
}

#[derive(Clone)]
pub struct Piece {
    pub h: usize,
    pub w: usize,
    pub cells: Vec<Vec<char>>, // '.' empty; anything else = filled
}

#[derive(Clone, Copy)]
pub struct Symbols {
    pub me_up: char,
    pub me_lo: char,
    pub op_up: char,
    pub op_lo: char,
}

impl Symbols {
    pub fn p1() -> Self {
        Self { me_up: '@', me_lo: 'a', op_up: '$', op_lo: 's' }
    }
    pub fn p2() -> Self {
        Self { me_up: '$', me_lo: 's', op_up: '@', op_lo: 'a' }
    }
    #[inline]
    pub fn is_op(&self, c: char) -> bool {
        c == self.op_up || c == self.op_lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_and_p2_wiring_is_correct() {
        let p1 = Symbols::p1();
        let p2 = Symbols::p2();
        assert_eq!(p1.me_up, '@');
        assert_eq!(p1.me_lo, 'a');
        assert_eq!(p1.op_up, '$');
        assert_eq!(p1.op_lo, 's');

        assert_eq!(p2.me_up, '$');
        assert_eq!(p2.me_lo, 's');
        assert_eq!(p2.op_up, '@');
        assert_eq!(p2.op_lo, 'a');
    }

    #[test]
    fn is_op_checks_both_cases() {
        let p1 = Symbols::p1();
        assert!(p1.is_op('$'));
        assert!(p1.is_op('s'));
        assert!(!p1.is_op('@'));
        assert!(!p1.is_op('a'));

        let p2 = Symbols::p2();
        assert!(p2.is_op('@'));
        assert!(p2.is_op('a'));
        assert!(!p2.is_op('$'));
        assert!(!p2.is_op('s'));
    }
}
