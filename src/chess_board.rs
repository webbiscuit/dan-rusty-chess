use crate::chess_move::ChessMove;
use crate::piece::Piece;
use colored::*;
use std::str;

pub type SquareIndex = u32;

pub struct ChessBoard {
    board: [Option<char>; 64],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        ChessBoard { board: [None; 64] }
    }
    fn to_board_index(file: SquareIndex, rank: SquareIndex) -> SquareIndex {
        return rank * 8 + file;
    }
    pub fn from_fen(fen: &str) -> ChessBoard {
        let mut board = ChessBoard::new();

        let mut fen_sections = fen.split(" ");

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
                let ix = ChessBoard::to_board_index(file, rank);
                board.board[ix as usize] = Some(piece_placement); //piece_placement;
                file += 1;
            }
        }

        return board;
    }
    fn get_rank(&self, rank: usize) -> &[Option<char>] {
        let start = (rank - 1) * 8;
        &self.board[start..start + 8]
    }

    pub fn draw(&self) -> String {
        let mut output: String = "".to_owned();

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

        return output;
    }

    pub fn square_from_notation(notation: &str) -> Option<SquareIndex> {
        let files = "abcdefgh";
        let ranks = "12345678";

        let file = notation.chars().nth(0);
        let rank = notation.chars().nth(1);

        let file_ix = files.find(file?);
        let rank_ix = ranks.find(rank?);

        let ix = Some((file_ix? + rank_ix? * 8).try_into().unwrap());

        return ix;
    }

    pub fn square_to_notation(index: SquareIndex) -> String {
        return "A1".to_string();
    }

    pub fn square_from_file_and_rank(file: SquareIndex, rank: SquareIndex) -> SquareIndex {
        return rank * 8 + file;
    }

    pub fn generate_moves(&self, index: SquareIndex) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        moves.push(ChessMove::new(1));
        return moves;
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
}
