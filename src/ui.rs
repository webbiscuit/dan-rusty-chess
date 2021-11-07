use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use crate::chess_board::{ChessBoard, SquareIndex, TOTAL_RANKS};
use crate::piece;

// use crate::app::App;

pub fn draw<B>(rect: &mut Frame<B>, chessboard: &ChessBoard)
where
    B: Backend,
{
    let size = rect.size();
    // check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(20)].as_ref())
        .split(size);

    // Add widgets
    let title = draw_title();
    let chessboard = draw_chessboard(chessboard);
    rect.render_widget(title, chunks[0]);
    rect.render_widget(chessboard, chunks[1]);
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

fn draw_chessboard<'a>(chessboard: &ChessBoard) -> Paragraph<'a> {
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
                piece_symbol = format!("  ");
            }

            let square_index: SquareIndex =
                ChessBoard::square_from_file_and_rank(i as u8, rank - 1).unwrap();
            if (i + (rank as usize)) % 2 == 0 {
                square_colour = Color::Rgb(168, 123, 80);
                // if self.is_highlit(square_index) {
                //     output.push_str(&format!("{}", coloured_symbol.on_truecolor(255, 189, 123)));
                // } else {
                // output.push_str(&format!("{}", coloured_symbol.on_truecolor(168, 123, 80)));
                // }
            } else {
                square_colour = Color::Rgb(100, 70, 25);

                // if self.is_highlit(square_index) {
                //     output.push_str(&format!("{}", coloured_symbol.on_truecolor(240, 179, 64)));
                // } else {
                // output.push_str(&format!("{}", coloured_symbol.on_truecolor(100, 70, 25)));
                // }
            }

            board_line.push(Span::styled(
                piece_symbol,
                Style::default().fg(piece_colour).bg(square_colour),
            ));
        }

        board_lines.push(Spans::from(board_line));
    }

    let chessboard_ui = Paragraph::new(board_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    );

    chessboard_ui
}

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 10 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}
