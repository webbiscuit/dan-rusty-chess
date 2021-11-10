use crate::{chess_board::SquareIndex, chess_move::ChessMove};

pub struct App {
    pub ui_buffer: String,
    available_moves: Vec<ChessMove>,
    selected_square: Option<SquareIndex>,
}

impl App {
    pub fn new() -> Self {
        App {
            ui_buffer: String::new(),
            available_moves: Vec::new(),
            selected_square: None,
        }
    }

    pub fn add_available_move(&mut self, available_move: &ChessMove) {
        self.available_moves.push(*available_move);
    }

    pub fn available_moves(&self) -> &Vec<ChessMove> {
        &self.available_moves
    }

    pub fn clear_input(&mut self) {
        self.ui_buffer.clear();
        self.available_moves.clear();
        self.selected_square = None;
    }

    pub fn set_selected_square(&mut self, square: u8) {
        self.selected_square = Some(square);
    }

    pub fn get_selected_square(&self) -> Option<SquareIndex> {
        self.selected_square
    }
}
