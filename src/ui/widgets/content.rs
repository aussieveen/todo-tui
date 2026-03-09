use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::state::{app::AppState, todo::Todo};

use crate::ui::styles::{block_border_style, completed_style, priority_style};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let visible = state.visible_todos();
    let mut lines: Vec<Line> = Vec::new();
    let mut last_priority: Option<Option<char>> = None; // outer None = not yet started

    for (pos, (_, todo)) in visible.iter().enumerate() {
        // F and None both fall into the backlog group
        let group = match todo.priority {
            Some('A'..='E') => todo.priority,
            _ => None,
        };
        if last_priority != Some(group) {
            if last_priority.is_some() {
                lines.push(Line::from("")); // blank separator between groups
            }
            lines.push(priority_header(group));
            last_priority = Some(group);
        }

        let is_dimmed = state.selected.is_some() && state.selected != Some(pos);
        lines.push(render_todo_line(todo, is_dimmed));
    }

    let total_lines = lines.len();
    let vp = area.height.saturating_sub(2) as usize;
    let offset = state.scroll_offset as usize;
    let has_above = total_lines > vp && offset > 0;
    let has_below = total_lines > offset + vp;

    let title = match (has_above, has_below) {
        (true, true) => " ↑ Todos ↓ ",
        (true, false) => " ↑ Todos ",
        (false, true) => " Todos ↓ ",
        (false, false) => " Todos ",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(block_border_style())
        .title(title);
    frame.render_widget(
        Paragraph::new(lines)
            .block(block)
            .scroll((state.scroll_offset, 0)),
        area,
    );
}

fn priority_header(priority: Option<char>) -> Line<'static> {
    let label = match priority {
        Some('A') => "Today",
        Some('B') => "This Week",
        Some('C') => "This Sprint",
        Some('D') => "This Month",
        Some('E') => "This Quarter",
        _ => "Backlog",
    };
    Line::from(Span::styled(
        format!(" {label}"),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}

fn render_todo_line(todo: &Todo, is_dimmed: bool) -> Line<'_> {
    let row_style = if is_dimmed {
        Style::default().fg(Color::DarkGray)
    } else if todo.done {
        completed_style()
    } else {
        priority_style(todo.priority)
    };

    let secondary_style = Style::default().fg(if is_dimmed {
        Color::DarkGray
    } else {
        Color::Gray
    });

    let marker = if todo.done { "✓ " } else { "  " };

    let desc_clean: String = todo
        .description
        .split_whitespace()
        .filter(|w| !w.starts_with('@') && !w.starts_with('+') && !w.starts_with("due:"))
        .collect::<Vec<_>>()
        .join(" ");

    let tags: String = todo
        .description
        .split_whitespace()
        .filter(|w| w.starts_with('@') || w.starts_with('+'))
        .collect::<Vec<_>>()
        .join(" ");

    let mut spans: Vec<Span> = vec![
        Span::styled(format!(" {marker}"), row_style),
        Span::styled(desc_clean, row_style),
    ];

    if !tags.is_empty() {
        spans.push(Span::styled(format!("  {tags}"), secondary_style));
    }

    let mut date_parts: Vec<String> = Vec::new();
    if let Some(d) = todo.creation_date {
        date_parts.push(format!("created:{}", d.format("%Y-%m-%d")));
    }
    if let Some(d) = todo.due_date {
        date_parts.push(format!("due:{}", d.format("%Y-%m-%d")));
    }
    if let Some(d) = todo.completion_date {
        date_parts.push(format!("done:{}", d.format("%Y-%m-%d")));
    }

    if !date_parts.is_empty() {
        spans.push(Span::styled(
            format!("  {}", date_parts.join("  ")),
            secondary_style,
        ));
    }

    Line::from(spans)
}
