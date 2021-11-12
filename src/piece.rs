use crate::{
    chess_board::{ChessBoard, SquareIndex},
    chess_move::{
        ChessMove, DiagonalSlidingMoves, JumpingMoves, MoveGenerator, PawnMoves,
        StraightSlidingMoves,
    },
};

pub struct Piece {
    symbol: char,
    move_generators: Vec<Box<dyn MoveGenerator>>,
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
    fn get_move_generators(symbol: char) -> Vec<Box<dyn MoveGenerator>> {
        match symbol {
            'R' | 'r' => vec![Box::new(StraightSlidingMoves::new(7))],
            'N' | 'n' => vec![Box::new(JumpingMoves::new(1))],
            'B' | 'b' => vec![Box::new(DiagonalSlidingMoves::new(7))],
            'Q' | 'q' => vec![
                Box::new(StraightSlidingMoves::new(7)),
                Box::new(DiagonalSlidingMoves::new(7)),
            ],
            'K' | 'k' => vec![
                Box::new(StraightSlidingMoves::new(1)),
                Box::new(DiagonalSlidingMoves::new(1)),
            ],
            'P' => vec![Box::new(PawnMoves::new(1, 2))],
            'p' => vec![Box::new(PawnMoves::new(-1, 7))],

            _ => vec![],
        }
    }
    pub fn is_black(self: &Piece) -> bool {
        self.symbol.is_lowercase()
    }
    pub fn is_white(self: &Piece) -> bool {
        !self.is_black()
    }

    pub fn generate_moves(
        self: &Piece,
        chess_board: &ChessBoard,
        source: SquareIndex,
    ) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        for move_generator in &self.move_generators {
            moves.append(&mut move_generator.generate_moves(chess_board, source));
        }
        moves
    }

    pub fn is_enemy(&self, other: &Piece) -> bool {
        self.is_black() == other.is_white()
    }
}
