use std::str;

struct ChessBoard {
    board: [u32; 64],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        ChessBoard { board: [0; 64] }
    }
    fn to_board_index(file: u32, rank: u32) -> u32 {
        return rank * 8 + file;
    }
    fn from_fen(fen: &str) -> ChessBoard {
        let mut board = ChessBoard { board: [0; 64] };

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
                board.board[ix as usize] = 1; //piece_placement;
                file += 1;
            }
        }

        return board;

        // self.board[1] = 1;
    }
    fn get_rank(&self, rank: usize) -> &[u32] {
        let start = (rank - 1) * 8;
        // println!("{} {}", rank, start);
        &self.board[start..start + 8]
    }

    pub fn draw(&self) -> String {
        let mut output: String = "".to_owned();
        // for square in self.board {
        //     output = output + "."
        // }
        // for (i, square) in self.board.iter().enumerate() {
        for rank in (1..9).rev() {
            for (i, square) in self.get_rank(rank).iter().enumerate() {
                if *square == 0u32 {
                    if (i + rank) % 2 == 0 {
                        output += "⬜"; //\u{0305}"
                    } else {
                        output += "⬛"
                    }
                } else {
                    //
                    output += "♞ "

                    //output += &square.to_string().to_owned();
                }
            }
            output += "\n";
        }
        // for (i, square) in self.get_rank(8).iter().enumerate() {
        //     output += &square.to_string()
        //     // if i >> 4 == 0 {
        //     //     output += "."
        //     // }

        //     // if i % 8 == 0 {
        //     //     output += "\n";
        //     //     continue;
        //     // }
        //     // output = output + &square.to_string()
        //     // output = output + &i.to_string()
        // }
        return output;
    }
}

impl Default for ChessBoard {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    // let chess_board = ChessBoard::new();
    let chess_board =
        ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let ui = chess_board.draw();

    println!("♞  Dan's Rusty Chess ♞\n");
    println!("I\u{0305}V\u{0305} - I̅V̅");
    println!("{}", ui);
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
