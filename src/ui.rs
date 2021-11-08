use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

use crate::app::App;
use crate::chess_board::{ChessBoard, SquareIndex, TOTAL_RANKS};
// use crate::app::App;

pub fn draw<B>(rect: &mut Frame<B>, app: &App, chessboard: &ChessBoard)
where
    B: Backend,
{
    let size = rect.size();
    // check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(12)].as_ref())
        .split(size);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(23), Constraint::Min(12)].as_ref())
        .split(chunks[1]);

    // Add widgets
    let title = draw_title();
    let chessboard = draw_chessboard(chessboard);
    let user_input = draw_user_input(app);
    rect.render_widget(title, chunks[0]);
    // rect.render_widget(main_chunks, chunks[1]);
    rect.render_widget(chessboard, main_chunks[0]);
    rect.render_widget(user_input, main_chunks[1]);

    rect.set_cursor(
        main_chunks[1].x + app.input.width() as u16 + 1,
        main_chunks[1].y + 2,
    )
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("♞  Dan's Rusty Chess ♞")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn draw_user_input(app: &App) -> Paragraph {
    let notationed_moves: Vec<String> = app
        .available_moves()
        .iter()
        .map(|m| ChessBoard::square_to_notation(m.destination).unwrap())
        .collect();

    Paragraph::new(vec![
        Spans::from("Enter square to show moves: ".to_string()),
        Spans::from(app.input.to_string()),
        Spans::from("Moves are: ".to_string()),
        Spans::from(notationed_moves.join(",")),
    ])
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
}

fn draw_chessboard(chessboard: &ChessBoard) -> Paragraph {
    let mut board_lines: Vec<Spans> = Vec::new();

    board_lines.push(Spans::from(vec![Span::raw("  a b c d e f g h")]));

    for rank in (1..=TOTAL_RANKS).rev() {
        let mut board_line: Vec<Span> = Vec::new();
        board_line.push(Span::raw(format!("{} ", rank)));

        for (i, piece) in chessboard
            .get_pieces_on_rank(rank.into())
            .iter()
            .enumerate()
        {
            let piece_symbol;
            let square_colour;
            let mut piece_colour = Color::White;

            if let Some(piece) = piece {
                piece_symbol = format!("{} ", piece.get_graphic());
                piece_colour = if piece.is_black() {
                    Color::Black
                } else {
                    Color::White
                };
            } else {
                piece_symbol = "  ".to_string();
            }

            let square_index: SquareIndex =
                ChessBoard::square_from_file_and_rank(i as u8, rank - 1).unwrap();
            if (i + (rank as usize)) % 2 == 0 {
                if chessboard.is_highlit(square_index) {
                    square_colour = Color::Rgb(255, 189, 123);
                } else {
                    square_colour = Color::Rgb(168, 123, 80);
                }
            } else {
                if chessboard.is_highlit(square_index) {
                    square_colour = Color::Rgb(240, 179, 64);
                } else {
                    square_colour = Color::Rgb(100, 70, 25);
                }
            }

            board_line.push(Span::styled(
                piece_symbol,
                Style::default().fg(piece_colour).bg(square_colour),
            ));
        }
        board_line.push(Span::raw(format!(" {}", rank)));

        board_lines.push(Spans::from(board_line));
    }
    board_lines.push(Spans::from(vec![Span::raw("  a b c d e f g h")]));

    let chessboard_ui = Paragraph::new(board_lines)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        );

    chessboard_ui
}

// fn check_size(rect: &Rect) {
//     if rect.width < 52 {
//         panic!("Require width >= 52, (got {})", rect.width);
//     }
//     if rect.height < 10 {
//         panic!("Require height >= 28, (got {})", rect.height);
//     }
// }
