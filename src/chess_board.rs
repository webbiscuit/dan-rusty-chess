use crate::{chess_move::ChessMove, piece::Piece};
use std::str;

pub type SquareIndex = u8;
pub type FileIndex = u8;
pub type RankIndex = u8;

pub const TOTAL_RANKS: RankIndex = 8;
pub const TOTAL_FILES: FileIndex = 8;
pub const TOTAL_SQUARES: SquareIndex = TOTAL_RANKS * TOTAL_FILES;

pub struct ChessBoard {
    board: [Option<Piece>; TOTAL_SQUARES as usize],
    highlit: [bool; TOTAL_SQUARES as usize],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        const INIT: Option<Piece> = None;
        ChessBoard {
            board: [INIT; TOTAL_SQUARES as usize],
            highlit: [false; TOTAL_SQUARES as usize],
        }
    }
    pub fn from_fen(fen: &str) -> ChessBoard {
        let mut board = ChessBoard::new();

        let mut fen_sections = fen.split(' ');

        let mut file: SquareIndex = 0;
        let mut rank: SquareIndex = TOTAL_RANKS - 1;

        let piece_placements = fen_sections.next().unwrap_or("");

        for piece_placement in piece_placements.chars() {
            if piece_placement.is_digit(10) {
                file += piece_placement.to_digit(10).unwrap_or_default() as u8
            } else if piece_placement == '/' {
                rank -= 1;
                file = 0;
            } else {
                let ix = ChessBoard::square_from_file_and_rank(file, rank).unwrap();
                let piece = Piece::new(piece_placement);
                board.board[ix as usize] = Some(piece);
                file += 1;
            }
        }

        board
    }
    pub fn get_piece(&self, square_index: SquareIndex) -> &Option<Piece> {
        &self.board[square_index as usize]
    }
    pub fn get_pieces_on_rank(&self, rank: usize) -> &[Option<Piece>] {
        let start = (rank - 1) * TOTAL_FILES as usize;
        &self.board[start..start + TOTAL_FILES as usize]
    }

    // pub fn draw(&self) -> String {
    //     let mut output: String = format!("");

    //     output.push_str("  a b c d e f g h\n");

    //     for rank in (1..=TOTAL_RANKS).rev() {
    //         output.push_str(&format!("{} ", rank));
    //         for (i, piece) in self.get_pieces_on_rank(rank.into()).iter().enumerate() {
    //             let coloured_symbol: ColoredString;

    //             if let Some(piece) = piece {
    //                 let piece_symbol = &format!("{} ", piece.get_graphic());
    //                 let piece_symbol = if piece.is_black() {
    //                     piece_symbol.truecolor(0, 0, 0)
    //                 } else {
    //                     piece_symbol.truecolor(240, 240, 240)
    //                 };
    //                 coloured_symbol = piece_symbol;
    //             } else {
    //                 let piece_symbol = &format!("{} ", " ");
    //                 coloured_symbol = piece_symbol.white();
    //             }

    //             let square_index: SquareIndex =
    //                 ChessBoard::square_from_file_and_rank(i as u8, rank - 1).unwrap();
    //             if (i + (rank as usize)) % 2 == 0 {
    //                 if self.is_highlit(square_index) {
    //                     output
    //                         .push_str(&format!("{}", coloured_symbol.on_truecolor(255, 189, 123)));
    //                 } else {
    //                     output.push_str(&format!("{}", coloured_symbol.on_truecolor(168, 123, 80)));
    //                 }
    //             } else {
    //                 if self.is_highlit(square_index) {
    //                     output.push_str(&format!("{}", coloured_symbol.on_truecolor(240, 179, 64)));
    //                 } else {
    //                     output.push_str(&format!("{}", coloured_symbol.on_truecolor(100, 70, 25)));
    //                 }
    //             }
    //         }
    //         output.push_str(&format!(" {} ", rank));

    //         output += "\n";
    //     }
    //     output.push_str("  a b c d e f g h");

    //     output
    // }

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
        let ix = rank * TOTAL_RANKS + file;

        if ix >= TOTAL_SQUARES {
            None
        } else {
            Some(ix)
        }
    }

    pub fn square_to_file_and_rank(square_index: SquareIndex) -> (FileIndex, RankIndex) {
        let file_ix = square_index % TOTAL_FILES;
        let rank_ix = square_index / TOTAL_RANKS;

        (file_ix, rank_ix)
    }

    pub fn generate_moves(&self, index: SquareIndex) -> Vec<ChessMove> {
        if let Some(piece) = self.get_piece(index) {
            piece.generate_moves(self, index)
        } else {
            vec![]
        }
    }

    pub fn reset_highlights(&mut self) {
        self.highlit = [false; TOTAL_SQUARES as usize];
    }

    pub fn highlight_square(&mut self, index: SquareIndex, highlight: bool) {
        self.highlit[index as usize] = highlight;
    }

    pub fn is_highlit(&self, index: SquareIndex) -> bool {
        self.highlit[index as usize]
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
        assert_eq!(ChessBoard::square_to_notation(255), None);
        assert_eq!(ChessBoard::square_to_notation(u8::MAX), None);
        assert_eq!(ChessBoard::square_to_notation(64), None);
    }

    #[test]
    fn test_to_and_from_file_ranks() {
        let square = ChessBoard::square_from_file_and_rank(1, 1).unwrap();
        assert_eq!("b2", ChessBoard::square_to_notation(square).unwrap());
    }
}
