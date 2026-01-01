use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

use super::super::app::App;
use super::super::theme;

pub(crate) fn render_envs(frame: &mut Frame, area: Rect, app: &mut App) {
    let outer = Block::default().borders(Borders::ALL).title("Environments");
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let active_name = app
        .env_config
        .as_ref()
        .and_then(|config| config.active.as_deref())
        .unwrap_or("<none>");

    let envs_dir = app
        .env_config
        .as_ref()
        .map(|config| config.envs_dir.display().to_string())
        .unwrap_or_else(|| app.workspace.envs_dir().display().to_string());
    let mut info_lines = vec![
        Line::from(format!("Dir: {}", envs_dir)),
        Line::from(format!("Active: {}", active_name)),
    ];
    let defaults_count = app
        .env_config
        .as_ref()
        .map(|config| config.defaults.len())
        .unwrap_or(0);
    info_lines.push(Line::from(format!("Defaults: {}", defaults_count)));
    if let Some(err) = &app.env_error {
        info_lines.push(Line::from(vec![
            Span::styled("Error: ", Style::default().fg(Color::Red)),
            Span::raw(err),
        ]));
    }
    let info_height = info_lines.len() as u16 + 2;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(info_height),
            Constraint::Min(3),
            Constraint::Length(2),
        ])
        .split(inner);

    let info = Paragraph::new(info_lines)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .wrap(Wrap { trim: true });
    frame.render_widget(info, chunks[0]);

    if app.env_entries.is_empty() {
        let empty = Paragraph::new("No environment files found.")
            .block(Block::default().borders(Borders::ALL).title("Files"))
            .wrap(Wrap { trim: true });
        frame.render_widget(empty, chunks[1]);
    } else {
        let items: Vec<ListItem> = app
            .env_entries
            .iter()
            .map(|entry| {
                let active = app
                    .env_config
                    .as_ref()
                    .and_then(|config| config.active.as_deref())
                    .map(|name| name == entry.name)
                    .unwrap_or(false);
                let label = if active {
                    format!("* {}", entry.name)
                } else {
                    format!("  {}", entry.name)
                };
                ListItem::new(Line::from(label))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Files"))
            .highlight_style(theme::selection_style())
            .highlight_symbol(theme::selection_symbol_str());
        frame.render_stateful_widget(list, chunks[1], &mut app.env_state);
    }

    let footer = Paragraph::new("Up/Down move, Enter activate, d deactivate, r reload, Esc/q back")
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(footer, chunks[2]);
}
