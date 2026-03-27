use chrono::Utc;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::state::app::{AppFocus, AppState};
use crate::sync::SyncStatus;
use crate::ui::styles::{key_desc_style, key_style};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let mut hints = match state.focus {
        AppFocus::Main => vec![
            hint("↑↓", "navigate"),
            hint("Shift ↑↓", "priority"),
            hint("a", "add"),
            hint("e/↵", "edit"),
            hint("x/spc", "complete"),
            hint("d", "delete"),
            hint("t", "completed"),
            hint("q/Esc", "quit"),
        ],
        AppFocus::Popup => vec![
            hint("Tab", "next field"),
            hint("Shift Tab", "prev field"),
            hint("↵", "save"),
            hint("Esc", "cancel"),
        ],
        AppFocus::ErrorPopup => vec![hint("d/Esc", "dismiss")],
        AppFocus::SyncConflict => vec![hint("d", "use Drive version"), hint("l", "keep local")],
    };

    // Append sync status indicator
    if let Some(status_span) = sync_status_span(&state.sync_status) {
        hints.push(Line::from(vec![Span::raw("  "), status_span]));
    }

    let available = area.width as usize;
    let mut line1_spans: Vec<Span> = Vec::new();
    let mut line2_spans: Vec<Span> = Vec::new();
    let mut line1_width = 0usize;
    let mut on_line2 = false;

    for h in hints {
        let hint_width: usize = h.spans.iter().map(|s| s.content.len()).sum();
        if !on_line2 && line1_width + hint_width > available {
            on_line2 = true;
        }
        if on_line2 {
            line2_spans.extend(h.spans);
        } else {
            line1_width += hint_width;
            line1_spans.extend(h.spans);
        }
    }

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(key_style())
        .title(" Help ")
        .title_alignment(Alignment::Center);

    frame.render_widget(
        Paragraph::new(vec![Line::from(line1_spans), Line::from(line2_spans)]).block(block),
        area,
    );
}

fn hint<'a>(key: &'a str, label: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!(" [{key}]"), key_style()),
        Span::styled(format!(" {label}"), key_desc_style()),
    ])
}

fn sync_status_span(status: &SyncStatus) -> Option<Span<'static>> {
    match status {
        SyncStatus::NotConfigured | SyncStatus::Idle => None,
        SyncStatus::Syncing => Some(Span::styled(
            "| syncing…".to_string(),
            Style::default().fg(Color::Yellow),
        )),
        SyncStatus::Synced(t) => {
            let ago = Utc::now().signed_duration_since(*t);
            let label = if ago.num_seconds() < 60 {
                "| synced <1m ago".to_string()
            } else {
                format!("| synced {}m ago", ago.num_minutes())
            };
            Some(Span::styled(label, Style::default().fg(Color::Green)))
        }
        SyncStatus::Offline => Some(Span::styled(
            "| offline".to_string(),
            Style::default().fg(Color::DarkGray),
        )),
        SyncStatus::Error(_) => Some(Span::styled(
            "| sync err".to_string(),
            Style::default().fg(Color::Red),
        )),
    }
}
