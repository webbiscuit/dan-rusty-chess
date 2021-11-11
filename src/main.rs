use crate::app::App;
use crate::chess_board::ChessBoard;

mod app;
mod chess_board;
mod chess_move;
mod piece;
mod ui;

// use clap::App as ClapApp;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::stdout;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{backend::CrosstermBackend, Terminal};

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .about("♞  Dan's Rusty Chess ♞")
        .arg(
            clap::Arg::with_name("fen")
                .short("f")
                .long("fen")
                .help("Start game with fen string")
                .takes_value(true),
        )
        .get_matches();

    let fen = matches
        .value_of("fen")
        .unwrap_or("rnbqkbnr/pppppppp/8/3nn3/8/PPPP4/4PPPP/RNBQKBNR w KQkq - 0 1");

    start_ui(fen)
}

pub fn start_ui(fen: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut chessboard = chess_board::ChessBoard::from_fen(fen);
    let mut app = App::new();

    // Configure Crossterm backend for tui
    let stdout = stdout();
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if tx.send(Event::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        }
    });

    loop {
        terminal.draw(|rect| ui::draw(rect, &app, &chessboard))?;

        match rx.recv()? {
            Event::Input(event) => match (event.modifiers, event.code) {
                (event::KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                    // disable_raw_mode()?;
                    // terminal.show_cursor()?;
                    break;
                }
                (event::KeyModifiers::NONE, KeyCode::Backspace) => {
                    app.ui_buffer.pop();
                }
                (event::KeyModifiers::NONE, KeyCode::Enter) => {
                    let square_notation = app.ui_buffer.clone();
                    app.clear_input();
                    chessboard.reset_highlights();
                    // app.input.clear();

                    if let Some(square) = ChessBoard::square_from_notation(&square_notation) {
                        let moves = chessboard.generate_moves(square);
                        app.set_selected_square(square);

                        moves.iter().for_each(|m| {
                            chessboard.highlight_square(m.destination, true);
                            app.add_available_move(m);
                        });
                    }
                }
                (event::KeyModifiers::NONE, KeyCode::Char(c)) => {
                    app.ui_buffer.push(c);
                }
                _ => {}
            },
            _ => {}
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
    disable_raw_mode()?;

    Ok(())
}

// fn main() {
//     // let chess_board = chess_board::ChessBoard::from_fen(
//     //     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
//     // );
//     // let chess_board = ChessBoard::from_fen("8/1b6/8/8/8/8/8/8 w KQkq - 0 1");
//     let mut chess_board =
//         chess_board::ChessBoard::from_fen("r1bqkb1r/8/8/8/8/8/8/R1BQKB1R w KQkq - 0 1");
//     // chess_board.highlight_square(11, true);
//     // chess_board.highlight_square(12, true);

//     let ui = chess_board.draw();

//     println!("♞  Dan's Rusty Chess ♞\n");
//     println!("{}", ui);

//     if let Some(square) = ChessBoard::square_from_notation("b7") {
//         let moves = chess_board.generate_moves(square);

//         let notationed_moves: Vec<String> = moves
//             .iter()
//             .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
//             .collect();

//         println!("{}", notationed_moves.join(","));
//     }
// }
