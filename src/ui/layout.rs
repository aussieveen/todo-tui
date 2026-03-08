use ratatui::layout::{Constraint, Layout, Rect};

pub struct Areas {
    pub header: Rect,
    pub content: Rect,
    pub footer: Rect,
}

pub fn main(area: Rect) -> Areas {
    let [header, content, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .areas(area);

    Areas {
        header,
        content,
        footer,
    }
}
