use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Wrap},
};

use crate::state::app::{PopupField, PopupMode, PopupState};
use crate::ui::styles::{active_field_style, inactive_field_style, popup_border_style};

pub fn render(frame: &mut Frame, area: Rect, popup: &PopupState) {
    let popup_area = centered_popup(area, 62, 16);

    let title = match popup.mode {
        PopupMode::Add => " New Todo ",
        PopupMode::Edit => " Edit Todo ",
    };

    let block = Block::bordered()
        .title(title)
        .border_type(BorderType::Rounded)
        .border_style(popup_border_style());

    let inner = block.inner(popup_area);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(block, popup_area);

    let fields = build_field_lines(popup);
    let paragraph = Paragraph::new(fields).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, inner);
}

fn build_field_lines(popup: &PopupState) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    lines.push(field_line(
        "Description",
        &popup.description,
        popup.field == PopupField::Description,
    ));
    lines.push(Line::from(""));

    lines.push(field_line(
        "Priority   ",
        &popup.priority,
        popup.field == PopupField::Priority,
    ));
    // Priority hint — always visible
    lines.push(Line::from(Span::styled(
        "             A=today  B=week  C=sprint  D=month  E=quarter  (none=backlog)",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    )));
    lines.push(Line::from(""));

    lines.push(field_line(
        "Context    ",
        &format!(
            "{}{}",
            if popup.context.is_empty() { "" } else { "@" },
            popup.context
        ),
        popup.field == PopupField::Context,
    ));
    lines.push(Line::from(""));

    lines.push(field_line(
        "Project    ",
        &format!(
            "{}{}",
            if popup.project.is_empty() { "" } else { "+" },
            popup.project
        ),
        popup.field == PopupField::Project,
    ));
    lines.push(Line::from(""));

    lines.push(field_line(
        "Due Date   ",
        &popup.due_date,
        popup.field == PopupField::DueDate,
    ));
    lines.push(Line::from(Span::styled(
        "             Format: YYYY-MM-DD",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    )));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(
        "  [Enter] Save   [Esc] Cancel   [Tab] Next field",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    )));

    lines
}

fn field_line(label: &str, value: &str, active: bool) -> Line<'static> {
    let label_style = if active {
        active_field_style().add_modifier(Modifier::BOLD)
    } else {
        inactive_field_style()
    };

    let value_style = if active {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let cursor = if active { "_" } else { "" };

    Line::from(vec![
        Span::styled(format!("  {label}: "), label_style),
        Span::styled(format!("{value}{cursor}"), value_style),
    ])
}

fn centered_popup(area: Rect, percent_x: u16, height: u16) -> Rect {
    let [vertical_area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    let [popup_area] = Layout::horizontal([Constraint::Percentage(percent_x)])
        .flex(Flex::Center)
        .areas(vertical_area);
    popup_area
}

pub fn render_error(frame: &mut Frame, area: Rect, message: &str) {
    let popup_area = centered_popup(area, 50, 6);

    let block = Block::bordered()
        .title(" Error ")
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Red));

    let inner = block.inner(popup_area);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(block, popup_area);

    let text = vec![
        Line::from(message.to_string()),
        Line::from(""),
        Line::from(Span::styled(
            "  Press [d] or [Esc] to dismiss",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), inner);
}
