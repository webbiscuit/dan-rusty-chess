use std::str;

struct ChessBoard {
    board: [Option<char>; 64],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        ChessBoard { board: [None; 64] }
    }
    fn to_board_index(file: u32, rank: u32) -> u32 {
        return rank * 8 + file;
    }
    fn from_fen(fen: &str) -> ChessBoard {
        let mut board = ChessBoard::new();

        let mut fen_sections = fen.split(" ");

        let mut file: u32 = 0;
        let mut rank: u32 = 7;

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
        match piece {
            'r' => '\u{265C}',
            'R' => '\u{2656}',
            'n' => '\u{265E}',
            'N' => '\u{2658}',
            'b' => '\u{265D}',
            'B' => '\u{2657}',
            'k' => '\u{265A}',
            'K' => '\u{2654}',
            'q' => '\u{265B}',
            'Q' => '\u{2655}',
            'p' => '\u{265F}',
            'P' => '\u{2659}',
            _ => 'X',
        }
    }

    pub fn draw(&self) -> String {
        let mut output: String = "".to_owned();

        output.push_str("  a b c d e f g h\n");

        for rank in (1..9).rev() {
            output.push_str(&format!("{} ", rank));
            for (i, square) in self.get_rank(rank).iter().enumerate() {
                if *square == None {
                    if (i + rank) % 2 == 0 {
                        output.push('⬜');
                    } else {
                        output.push('⬛');
                    }
                } else {
                    let piece = square.unwrap();

                    output.push(ChessBoard::get_piece_graphic(piece));
                    output.push(' ');
                }
            }
            output.push_str(&format!(" {} ", rank));

            output += "\n";
        }
        output.push_str("  a b c d e f g h");

        return output;
    }
}

impl Default for ChessBoard {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let chess_board =
        ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let ui = chess_board.draw();

    println!("♞  Dan's Rusty Chess ♞\n");
    println!("{}", ui);
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
