use crate::chess_board::{ChessBoard, SquareIndex, TOTAL_FILES, TOTAL_RANKS};

pub struct ChessMove {
    pub source: SquareIndex,
    pub destination: SquareIndex,
}

pub trait MoveGenerator {
    fn generate_moves(&self, chess_board: &ChessBoard, source: SquareIndex) -> Vec<ChessMove>;
}

pub struct StraightSlidingMoves {
    max_moves: u8,
}
impl StraightSlidingMoves {
    pub fn new(max_moves: u8) -> Self {
        Self { max_moves }
    }
}
impl MoveGenerator for StraightSlidingMoves {
    fn generate_moves(&self, chess_board: &ChessBoard, source: SquareIndex) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();

        let (file_ix, rank_ix) = ChessBoard::square_to_file_and_rank(source);

        for f in 0..=self.max_moves + 1 {
            let delta: i8 = (f as i8) - (file_ix as i8);

            if delta == 0 {
                continue;
            }

            for (df, dr) in &[(1, 0), (0, -1)] {
                let file: i8 = (file_ix as i8) + (df * delta);
                let rank: i8 = (rank_ix as i8) + (dr * delta);

                if (rank >= (TOTAL_RANKS as i8) || rank < 0)
                    || (file >= (TOTAL_FILES as i8) || file < 0)
                {
                    continue;
                }

                let destination = ChessBoard::square_from_file_and_rank(
                    file.try_into().unwrap(),
                    rank.try_into().unwrap(),
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

pub struct DiagonalSlidingMoves {
    max_moves: u8,
}
impl DiagonalSlidingMoves {
    pub fn new(max_moves: u8) -> Self {
        Self { max_moves }
    }
}
impl MoveGenerator for DiagonalSlidingMoves {
    fn generate_moves(&self, chess_board: &ChessBoard, source: SquareIndex) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();

        let (file_ix, rank_ix) = ChessBoard::square_to_file_and_rank(source);

        for f in 0..=self.max_moves + 1 {
            let delta: i8 = (f as i8) - (file_ix as i8);

            if delta == 0 {
                continue;
            }

            for (df, dr) in &[(1, 1), (1, -1)] {
                let file: i8 = (file_ix as i8) + (df * delta);
                let rank: i8 = (rank_ix as i8) + (dr * delta);

                if (rank >= (TOTAL_RANKS as i8) || rank < 0)
                    || (file >= (TOTAL_FILES as i8) || file < 0)
                {
                    continue;
                }

                let destination = ChessBoard::square_from_file_and_rank(
                    file.try_into().unwrap(),
                    rank.try_into().unwrap(),
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

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec![
            "a7", "c7", "d7", "e7", "f7", "g7", "h7", "b1", "b2", "b3", "b4", "b5", "b6", "b8",
        ];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
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

    #[test]
    fn test_king_moves() {
        let chess_board = ChessBoard::from_fen("8/1k6/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("b7").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["a6", "a7", "a8", "b6", "b8", "c6", "c7", "c8"];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

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

    #[test]
    fn test_more_queen_moves() {
        let chess_board = ChessBoard::from_fen("8/8/8/8/8/8/8/7Q w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("h1").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec![
            "a1", "a8", "b1", "b7", "c1", "c6", "d1", "d5", "e1", "e4", "f1", "f3", "g1", "g2",
            "h2", "h3", "h4", "h5", "h6", "h7", "h8",
        ];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_no_moves_for_empty_squares() {
        let chess_board = ChessBoard::from_fen("8/8/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("a1").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves: Vec<String> = vec![];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }
}
