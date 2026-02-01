use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};

use super::super::app::ExecutionStatus;

pub(crate) fn status_label_and_style(status: &ExecutionStatus) -> (String, Style) {
    match status {
        ExecutionStatus::Success => ("OK".to_string(), Style::default().fg(Color::Green)),
        ExecutionStatus::Failed(code) => match code {
            Some(code) => (format!("FAIL ({})", code), Style::default().fg(Color::Red)),
            None => ("FAIL".to_string(), Style::default().fg(Color::Red)),
        },
        ExecutionStatus::Error => ("ERROR".to_string(), Style::default().fg(Color::Yellow)),
    }
}

pub(crate) fn standard_screen_layout(
    area: Rect,
    header_height: u16,
    footer_height: u16,
) -> [Rect; 3] {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header_height),
            Constraint::Min(3),
            Constraint::Length(footer_height),
        ])
        .split(area);

    [chunks[0], chunks[1], chunks[2]]
}

pub(crate) fn horizontal_split(area: Rect, left_percent: u16) -> [Rect; 2] {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(left_percent),
            Constraint::Percentage(100 - left_percent),
        ])
        .split(area);

    [chunks[0], chunks[1]]
}
