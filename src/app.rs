use crate::chess_move::ChessMove;

pub struct App {
    pub input: String,
    available_moves: Vec<ChessMove>,
}

impl App {
    pub fn new() -> Self {
        App {
            input: String::new(),
            available_moves: Vec::new(),
        }
    }

    pub fn add_available_move(self: &mut Self, available_move: &ChessMove) {
        self.available_moves.push(*available_move);
    }

    pub fn available_moves(self: &Self) -> &Vec<ChessMove> {
        &self.available_moves
    }

    pub fn clear_input(self: &mut Self) {
        self.input.clear();
        self.available_moves.clear();
    }
}
