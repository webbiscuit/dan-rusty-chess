use crate::{
    chess_board::{ChessBoard, SquareIndex},
    chess_move::{ChessMove, DiagonalSlidingMoves, MoveGenerator, StraightSlidingMoves},
};

type FnPtr = fn(chess_board: &ChessBoard, source: SquareIndex) -> Vec<ChessMove>;

#[derive(Clone)]
pub struct Piece {
    symbol: char,
    move_generators: Vec<FnPtr>,
}

impl Piece {
    pub fn new(symbol: char) -> Piece {
        Piece {
            symbol,
            move_generators: Piece::get_move_generators(symbol),
        }
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
    fn get_move_generators(symbol: char) -> Vec<FnPtr> {
        match symbol {
            'R' | 'r' => vec![StraightSlidingMoves::generate_moves],
            'N' | 'n' => vec![],
            'B' | 'b' => vec![DiagonalSlidingMoves::generate_moves],
            'Q' | 'q' => vec![
                StraightSlidingMoves::generate_moves,
                DiagonalSlidingMoves::generate_moves,
            ],
            'K' | 'k' => vec![],
            'P' | 'p' => vec![],

            _ => vec![],
        }
    }
    pub fn is_black(self: &Piece) -> bool {
        self.symbol.is_lowercase()
    }

    pub fn generate_moves(
        self: &Piece,
        chess_board: &ChessBoard,
        source: SquareIndex,
    ) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        for move_generator in &self.move_generators {
            moves.append(&mut (move_generator)(chess_board, source));
        }
        moves
    }
}
