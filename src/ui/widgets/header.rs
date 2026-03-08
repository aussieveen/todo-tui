use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn render(frame: &mut Frame, area: Rect, todo_count: usize, completed_count: usize) {
    let line = Line::from(vec![
        Span::styled(
            " todo-tui",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  {todo_count} pending"),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            format!("  {completed_count} completed"),
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    frame.render_widget(Paragraph::new(line), area);
}
