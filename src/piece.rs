pub struct Piece {
    symbol: char,
    // move_generator
}

impl Piece {
    pub fn new(symbol: char) -> Piece {
        Piece { symbol }
    }

    pub fn get_graphic(self: &Piece) -> char {
        match self.symbol.to_ascii_uppercase() {
            'R' => '\u{265C}',
            'r' => '\u{2656}',
            'N' => '\u{265E}',
            'n' => '\u{2658}',
            'B' => '\u{265D}',
            'b' => '\u{2657}',
            'K' => '\u{265A}',
            'k' => '\u{2654}',
            'Q' => '\u{265B}',
            'q' => '\u{2655}',
            'P' => '\u{265F}',
            'p' => '\u{2659}',
            _ => ' ',
        }
    }
    pub fn is_black(self: &Piece) -> bool {
        self.symbol.is_lowercase()
    }
}
