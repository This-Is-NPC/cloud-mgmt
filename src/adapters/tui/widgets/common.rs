use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;

use super::super::app::ExecutionStatus;
use super::super::theme::Theme;

pub(crate) fn status_label_and_style(status: &ExecutionStatus, theme: &Theme) -> (String, Style) {
    match status {
        ExecutionStatus::Success => ("OK".to_string(), theme.status_ok_style()),
        ExecutionStatus::Failed(code) => match code {
            Some(code) => (format!("FAIL ({})", code), theme.status_fail_style()),
            None => ("FAIL".to_string(), theme.status_fail_style()),
        },
        ExecutionStatus::Error => ("ERROR".to_string(), theme.status_error_style()),
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
