use crate::chess_board::{ChessBoard, SquareIndex};

pub struct ChessMove {
    pub source: SquareIndex,
    pub destination: SquareIndex,
}

pub trait MoveGenerator {
    fn generate_moves(chess_board: &ChessBoard, source: SquareIndex) -> Vec<ChessMove>;
}

pub struct StraightSlidingMoves;
impl MoveGenerator for StraightSlidingMoves {
    fn generate_moves(chess_board: &ChessBoard, source: SquareIndex) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();

        let (file_ix, rank_ix) = ChessBoard::square_to_file_and_rank(source);

        for f in 0..8 {
            if f == file_ix {
                continue;
            }

            let chess_move = ChessMove {
                source,
                destination: ChessBoard::square_from_file_and_rank(f, rank_ix),
            };
            moves.push(chess_move);
        }

        for r in 0..8 {
            if r == rank_ix {
                continue;
            }

            let chess_move = ChessMove {
                source,
                destination: ChessBoard::square_from_file_and_rank(file_ix, r),
            };
            moves.push(chess_move);
        }

        moves
    }
}

// impl ChessMove {
//     pub fn new(destination: SquareIndex) -> ChessMove {
//         ChessMove { destination }
//     }

//     pub fn destination(&self) -> SquareIndex {
//         self.destination
//     }
// }
