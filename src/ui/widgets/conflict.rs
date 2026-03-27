use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::state::app::ConflictState;

pub fn render(frame: &mut Frame, area: Rect, conflict: &ConflictState) {
    let popup_area = centered_rect(70, 60, area);

    // Clear background
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Sync Conflict ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // message
            Constraint::Length(1), // spacer
            Constraint::Min(1),    // content diff summary
            Constraint::Length(2), // key hints
        ])
        .split(inner);

    // Header message
    let msg = Paragraph::new(vec![
        Line::from("Both local and Drive changed since the last sync."),
        Line::from(Span::styled(
            "Conflict logged to ~/.todo/conflicts/",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .alignment(Alignment::Center);
    frame.render_widget(msg, chunks[0]);

    // Summary: line counts on each side
    let local_lines = conflict.local_content.lines().count();
    let drive_lines = conflict.drive_content.lines().count();
    let summary = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Local:  ", Style::default().fg(Color::Cyan)),
            Span::raw(format!("{local_lines} todo(s)")),
        ]),
        Line::from(vec![
            Span::styled("Drive:  ", Style::default().fg(Color::Magenta)),
            Span::raw(format!("{drive_lines} todo(s)")),
        ]),
    ])
    .wrap(Wrap { trim: true });
    frame.render_widget(summary, chunks[2]);

    // Key hints
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(
            " [d] ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Use Drive version    "),
        Span::styled(
            " [l] ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Keep local"),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(hints, chunks[3]);
}

/// Returns a centred rect using `percent_x`/`percent_y` of the given area.
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area)[1];

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical)[1]
}
