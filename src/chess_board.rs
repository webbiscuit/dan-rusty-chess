use crate::chess_move::ChessMove;
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
    fn get_piece_graphic(piece: char) -> char {
        match piece.to_ascii_uppercase() {
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
    fn is_piece_black(piece: char) -> bool {
        piece.is_lowercase()
    }

    pub fn draw(&self) -> String {
        let mut output: String = "".to_owned();

        output.push_str("  a b c d e f g h\n");

        for rank in (1..=8).rev() {
            output.push_str(&format!("{} ", rank));
            for (i, square) in self.get_rank(rank).iter().enumerate() {
                let piece_char = square.unwrap_or_default();
                let piece_symbol = &format!("{} ", ChessBoard::get_piece_graphic(piece_char));
                let piece_symbol = if ChessBoard::is_piece_black(piece_char) {
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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
