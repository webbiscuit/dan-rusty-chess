mod chess_board;
mod chess_move;

fn main() {
    let chess_board = chess_board::ChessBoard::from_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    );
    let ui = chess_board.draw();

    println!("♞  Dan's Rusty Chess ♞\n");
    println!("{}", ui);
}
