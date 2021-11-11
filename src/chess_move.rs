use crate::chess_board::{ChessBoard, SquareIndex, TOTAL_FILES, TOTAL_RANKS};

#[derive(Clone, Copy)]

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
    fn generate_moves(&self, _chess_board: &ChessBoard, source: SquareIndex) -> Vec<ChessMove> {
        static DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        generate_moves(source, self.max_moves, &DIRECTIONS)
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
    fn generate_moves(&self, _chess_board: &ChessBoard, source: SquareIndex) -> Vec<ChessMove> {
        static DIRECTIONS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        generate_moves(source, self.max_moves, &DIRECTIONS)
    }
}

pub struct JumpingMoves {
    max_moves: u8,
}
impl JumpingMoves {
    pub fn new(max_moves: u8) -> Self {
        Self { max_moves }
    }
}
impl MoveGenerator for JumpingMoves {
    fn generate_moves(&self, _chess_board: &ChessBoard, source: SquareIndex) -> Vec<ChessMove> {
        static DIRECTIONS: [(i8, i8); 8] = [
            (1, 2),
            (-1, -2),
            (2, 1),
            (-2, -1),
            (1, -2),
            (-1, 2),
            (2, -1),
            (-2, 1),
        ];

        generate_moves(source, self.max_moves, &DIRECTIONS)
    }
}

fn generate_moves(source: u8, max_moves: u8, directions: &[(i8, i8)]) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();
    let (file_ix, rank_ix) = ChessBoard::square_to_file_and_rank(source);

    for (df, dr) in directions {
        let mut file_ix = file_ix as i8 + df;
        let mut rank_ix = rank_ix as i8 + dr;

        let mut count = 0;
        while file_ix >= 0
            && file_ix < TOTAL_FILES as i8
            && rank_ix >= 0
            && rank_ix < TOTAL_RANKS as i8
        {
            let destination = ChessBoard::square_from_file_and_rank(file_ix as u8, rank_ix as u8);

            if let Some(destination) = destination {
                let chess_move = ChessMove {
                    source,
                    destination,
                };

                moves.push(chess_move);
            }

            count += 1;
            if count == max_moves {
                break;
            }
            file_ix += df;
            rank_ix += dr;
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_moves_from_a8() {
        let chess_board = ChessBoard::from_fen("r7/8/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("a8").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec![
            "b8", "c8", "d8", "e8", "f8", "g8", "h8", "a7", "a6", "a5", "a4", "a3", "a2", "a1",
        ];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_rook_moves_from_a1() {
        let chess_board = ChessBoard::from_fen("8/8/8/8/8/8/8/r7 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("a1").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec![
            "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "a3", "a4", "a5", "a6", "a7", "a8",
        ];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_rook_moves_from_h8() {
        let chess_board = ChessBoard::from_fen("7r/8/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("h8").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec![
            "g8", "f8", "e8", "d8", "c8", "b8", "a8", "h7", "h6", "h5", "h4", "h3", "h2", "h1",
        ];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_rook_moves_from_h1() {
        let chess_board = ChessBoard::from_fen("8/8/8/8/8/8/8/7r w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("h1").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec![
            "g1", "f1", "e1", "d1", "c1", "b1", "a1", "h2", "h3", "h4", "h5", "h6", "h7", "h8",
        ];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_bishop_moves_from_a8() {
        let chess_board = ChessBoard::from_fen("b7/8/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("a8").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["b7", "c6", "d5", "e4", "f3", "g2", "h1"];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_bishop_moves_from_h8() {
        let chess_board = ChessBoard::from_fen("7b/8/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("h8").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["g7", "f6", "e5", "d4", "c3", "b2", "a1"];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_bishop_moves_from_a1() {
        let chess_board = ChessBoard::from_fen("8/8/8/8/8/8/8/b7 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("a1").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["b2", "c3", "d4", "e5", "f6", "g7", "h8"];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_bishop_moves_from_h1() {
        let chess_board = ChessBoard::from_fen("8/8/8/8/8/8/8/7b w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("h1").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["g2", "f3", "e4", "d5", "c6", "b7", "a8"];
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
    fn test_king_moves_from_e1() {
        let chess_board = ChessBoard::from_fen("8/8/8/8/8/8/8/4K3 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("e1").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["d1", "d2", "e2", "f2", "f1"];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_king_moves_from_e8() {
        let chess_board = ChessBoard::from_fen("4k3/8/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("e8").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["d8", "f8", "d7", "e7", "f7"];
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

    #[test]
    fn test_knight_moves_from_a8() {
        let chess_board = ChessBoard::from_fen("n7/8/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("a8").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["c7", "b6"];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_knight_moves_from_h8() {
        let chess_board = ChessBoard::from_fen("7n/8/8/8/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("h8").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["f7", "g6"];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_knight_moves_from_a1() {
        let chess_board = ChessBoard::from_fen("8/8/8/8/8/8/8/n7 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("a1").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["c2", "b3"];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_knight_moves_from_h1() {
        let chess_board = ChessBoard::from_fen("8/8/8/8/8/8/8/7n w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("h1").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["f2", "g3"];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }

    #[test]
    fn test_knight_moves_from_d5() {
        let chess_board = ChessBoard::from_fen("8/8/8/3n4/8/8/8/8 w KQkq - 0 1");
        let square = ChessBoard::square_from_notation("d5").unwrap();
        let moves = chess_board.generate_moves(square);

        let mut notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        notationed_moves.sort();
        let mut expected_moves = vec!["c7", "e7", "c3", "e3", "b6", "b4", "f6", "f4"];
        expected_moves.sort();

        assert_eq!(notationed_moves, expected_moves)
    }
}
