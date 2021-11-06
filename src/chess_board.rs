use crate::{
    chess_move::{ChessMove, DiagonalSlidingMoves, MoveGenerator},
    piece::Piece,
};
use colored::*;
use std::str;

pub type SquareIndex = u32;
pub type FileIndex = u32;
pub type RankIndex = u32;

pub struct ChessBoard {
    board: [Option<char>; 64],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        ChessBoard { board: [None; 64] }
    }
    pub fn from_fen(fen: &str) -> ChessBoard {
        let mut board = ChessBoard::new();

        let mut fen_sections = fen.split(' ');

        let mut file: SquareIndex = 0;
        let mut rank: SquareIndex = 7;

        let piece_placements = fen_sections.next().unwrap_or("");

        for piece_placement in piece_placements.chars() {
            if piece_placement.is_digit(10) {
                file += piece_placement.to_digit(10).unwrap_or_default()
            } else if piece_placement == '/' {
                rank -= 1;
                file = 0;
            } else {
                let ix = ChessBoard::square_from_file_and_rank(file, rank).unwrap();
                board.board[ix as usize] = Some(piece_placement);
                file += 1;
            }
        }

        board
    }
    fn get_rank(&self, rank: usize) -> &[Option<char>] {
        let start = (rank - 1) * 8;
        &self.board[start..start + 8]
    }

    pub fn draw(&self) -> String {
        let mut output: String = format!("");

        output.push_str("  a b c d e f g h\n");

        for rank in (1..=8).rev() {
            output.push_str(&format!("{} ", rank));
            for (i, square) in self.get_rank(rank).iter().enumerate() {
                let piece = Piece::new(square.unwrap_or_default());
                let piece_symbol = &format!("{} ", piece.get_graphic());
                let piece_symbol = if piece.is_black() {
                    piece_symbol.truecolor(0, 0, 0)
                } else {
                    piece_symbol.truecolor(240, 240, 240)
                };

                if (i + rank) % 2 == 0 {
                    output.push_str(&format!("{}", piece_symbol.on_truecolor(168, 123, 80)));
                } else {
                    output.push_str(&format!("{}", piece_symbol.on_truecolor(100, 70, 25)));
                }
            }
            output.push_str(&format!(" {} ", rank));

            output += "\n";
        }
        output.push_str("  a b c d e f g h");

        output
    }

    pub fn square_from_notation(notation: &str) -> Option<SquareIndex> {
        let files = "abcdefgh";
        let ranks = "12345678";
        let mut notation_chars = notation.chars();

        let file = notation_chars.next();
        let rank = notation_chars.next();

        let file_ix = files.find(file?);
        let rank_ix = ranks.find(rank?);

        Some((file_ix? + rank_ix? * 8).try_into().unwrap())
    }

    pub fn square_to_notation(index: SquareIndex) -> Option<String> {
        let files = "abcdefgh";
        let ranks = "12345678";

        let (file_ix, rank_ix) = ChessBoard::square_to_file_and_rank(index);

        let file = files.chars().nth(file_ix.try_into().unwrap());
        let rank = ranks.chars().nth(rank_ix.try_into().unwrap());

        Some(format!("{}{}", file?, rank?))
    }

    pub fn square_from_file_and_rank(file: FileIndex, rank: RankIndex) -> Option<SquareIndex> {
        let ix = rank * 8 + file;

        if ix >= 8 * 8 {
            None
        } else {
            Some(ix)
        }
    }

    pub fn square_to_file_and_rank(square_index: SquareIndex) -> (FileIndex, RankIndex) {
        let file_ix = square_index % 8;
        let rank_ix = square_index / 8;

        (file_ix, rank_ix)
    }

    pub fn generate_moves(&self, index: SquareIndex) -> Vec<ChessMove> {
        // let mover = StraightSlidingMoves {};

        // StraightSlidingMoves::generate_moves(self, index)
        DiagonalSlidingMoves::generate_moves(self, index)

        // mover.generate_moves(self, index);
        // let mut moves: Vec<ChessMove> = Vec::new();
        // moves.push(ChessMove::new(1));
        // return moves;
    }
}

impl Default for ChessBoard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_notation() {
        assert_eq!(ChessBoard::square_from_notation("a1"), Some(0));
        assert_eq!(ChessBoard::square_from_notation("a8"), Some(56));
        assert_eq!(ChessBoard::square_from_notation("h1"), Some(7));
        assert_eq!(ChessBoard::square_from_notation("h8"), Some(63));
        assert_eq!(ChessBoard::square_from_notation("dan"), None);
        assert_eq!(ChessBoard::square_from_notation("123"), None);
        assert_eq!(ChessBoard::square_from_notation(""), None);
    }

    #[test]
    fn test_to_notation() {
        assert_eq!(ChessBoard::square_to_notation(0), Some("a1".to_string()));
        assert_eq!(ChessBoard::square_to_notation(56), Some("a8".to_string()));
        assert_eq!(ChessBoard::square_to_notation(7), Some("h1".to_string()));
        assert_eq!(ChessBoard::square_to_notation(63), Some("h8".to_string()));
        assert_eq!(ChessBoard::square_to_notation(999), None);
        assert_eq!(ChessBoard::square_to_notation(u32::MAX), None);
        assert_eq!(ChessBoard::square_to_notation(64), None);
    }

    #[test]
    fn test_to_and_from_file_ranks() {
        let square = ChessBoard::square_from_file_and_rank(1, 1).unwrap();
        assert_eq!("b2", ChessBoard::square_to_notation(square).unwrap());
        // assert_eq!(ChessBoard::square_from_notation("a1"), Some(0));
        // assert_eq!(ChessBoard::square_from_notation("a8"), Some(56));
        // assert_eq!(ChessBoard::square_from_notation("h1"), Some(7));
        // assert_eq!(ChessBoard::square_from_notation("h8"), Some(63));
        // assert_eq!(ChessBoard::square_from_notation("dan"), None);
        // assert_eq!(ChessBoard::square_from_notation("123"), None);
        // assert_eq!(ChessBoard::square_from_notation(""), None);
    }
}
