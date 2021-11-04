// use crate::chess_board::SquareIndex;
// use crate::;
use colored::*;

mod chess_board;
mod chess_move;
mod square_index_finder;

fn main() {
    let chess_board = chess_board::ChessBoard::from_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    );
    // let chess_board = chess_board::ChessBoard::from_fen("r7/8/8/8/8/8/8/8 w KQkq - 0 1");
    let ui = chess_board.draw();

    println!("♞  Dan's Rusty Chess ♞\n");
    println!("{}", ui);

    // 'N' => '\u{265E}',
    //         'n' => '\u{2658}',

    // let moves = chess_board.generate_moves(square_index_finder::from_notation("a8"));

    // for chess_move in moves {
    //     println!(
    //         "{}",
    //         square_index_finder::to_notation(chess_move.destination())
    //     );
    // }
}
