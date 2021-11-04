use crate::chess_board::SquareIndex;

pub struct ChessMove {
    destination: SquareIndex,
}

impl ChessMove {
    pub fn new(destination: SquareIndex) -> ChessMove {
        ChessMove { destination }
    }

    pub fn destination(&self) -> SquareIndex {
        self.destination
    }
}
