use ratatui::style::{Color, Modifier, Style};

pub fn key_style() -> Style {
    Style::default().fg(Color::Cyan)
}

pub fn key_desc_style() -> Style {
    Style::default().add_modifier(Modifier::DIM)
}

pub fn block_border_style() -> Style {
    Style::default().fg(Color::Cyan)
}

pub fn priority_style(priority: Option<char>) -> Style {
    match priority {
        Some('A') => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        Some('B') => Style::default().fg(Color::Yellow),
        Some('C') => Style::default().fg(Color::Green),
        Some('D') => Style::default().fg(Color::Blue),
        Some('E') => Style::default().fg(Color::Magenta),
        _ => Style::default(),
    }
}

pub fn completed_style() -> Style {
    Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::CROSSED_OUT)
}

pub fn popup_border_style() -> Style {
    Style::default().fg(Color::Cyan)
}

pub fn active_field_style() -> Style {
    Style::default().fg(Color::Yellow)
}

pub fn inactive_field_style() -> Style {
    Style::default().fg(Color::Gray)
}
