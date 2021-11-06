use crate::chess_board::ChessBoard;

mod chess_board;
mod chess_move;
mod piece;

fn main() {
    // let chess_board = chess_board::ChessBoard::from_fen(
    //     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    // );
    let chess_board = ChessBoard::from_fen("8/1b6/8/8/8/8/8/8 w KQkq - 0 1");
    let ui = chess_board.draw();

    println!("♞  Dan's Rusty Chess ♞\n");
    println!("{}", ui);

    if let Some(square) = ChessBoard::square_from_notation("b7") {
        let moves = chess_board.generate_moves(square);

        let notationed_moves: Vec<String> = moves
            .iter()
            .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
            .collect();

        println!("{}", notationed_moves.join(","));
    }
}
