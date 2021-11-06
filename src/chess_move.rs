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
                destination: ChessBoard::square_from_file_and_rank(f, rank_ix).unwrap(),
            };
            moves.push(chess_move);
        }

        for r in 0..8 {
            if r == rank_ix {
                continue;
            }

            let chess_move = ChessMove {
                source,
                destination: ChessBoard::square_from_file_and_rank(file_ix, r).unwrap(),
            };
            moves.push(chess_move);
        }

        moves
    }
}

pub struct DiagonalSlidingMoves;
impl MoveGenerator for DiagonalSlidingMoves {
    fn generate_moves(chess_board: &ChessBoard, source: SquareIndex) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();

        let (file_ix, rank_ix) = ChessBoard::square_to_file_and_rank(source);

        for f in 0..8 {
            let delta = f - (file_ix as i8);

            if delta == 0 {
                continue;
            }

            for (df, dr) in [(1, 1), (1, -1)] {
                let destination = ChessBoard::square_from_file_and_rank(
                    ((file_ix as i8) + (df * delta)).try_into().unwrap(),
                    ((rank_ix as i8) + (dr * delta)).try_into().unwrap(),
                );

                if destination.is_some() {
                    let chess_move = ChessMove {
                        source,
                        destination: destination.unwrap(),
                    };

                    moves.push(chess_move);
                }
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_moves() {
        let chess_board = ChessBoard::from_fen("8/1r6/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("b7").unwrap();
        let moves = chess_board.generate_moves(square);

        let notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        assert_eq!(
            notationed_moves,
            vec![
                "a7", "c7", "d7", "e7", "f7", "g7", "h7", "b1", "b2", "b3", "b4", "b5", "b6", "b8"
            ]
        )
    }

    #[test]
    fn test_bishop_moves() {
        let chess_board = ChessBoard::from_fen("8/1b6/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("b7").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["a8", "c6", "d5", "e4", "f3", "g2", "h1", "c8", "a6"];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    // #[test]
    // fn test_king_moves() {
    //     let chess_board = ChessBoard::from_fen("8/1k6/8/8/8/8/8/8 w KQkq - 0 1");
    //     let square = ChessBoard::square_from_notation("b7").unwrap();
    //     let moves = chess_board.generate_moves(square);

    //     let notationed_moves: Vec<String> = moves
    //         .iter()
    //         .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
    //         .collect();

    //     assert_eq!(
    //         notationed_moves,
    //         vec!["a1", "a2", "a3", "b1", "b3", "c1", "c2", "c3"]
    //     )
    // }

    #[test]
    fn test_queen_moves() {
        let chess_board = ChessBoard::from_fen("8/1q6/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("b7").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec![
            "a7", "c7", "d7", "e7", "f7", "g7", "h7", "b1", "b2", "b3", "b4", "b5", "b6", "b8",
            "a8", "c6", "d5", "e4", "f3", "g2", "h1", "c8", "a6",
        ];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
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
