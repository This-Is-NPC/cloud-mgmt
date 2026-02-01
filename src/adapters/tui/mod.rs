mod app;
mod events;
mod state;
pub(crate) mod theme;
mod ui;
mod widgets;

use crate::search_index::SearchIndex;
use crate::use_cases::ScriptService;
use crate::workspace::Workspace;
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io;
use std::time::Duration;

use crate::history;
use app::{App, Screen};
use events::handle_key_event;
use theme::load_theme;
use ui::{render_loading, render_ui};

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    service: &ScriptService,
    workspace: Workspace,
) -> Result<(), Box<dyn Error>> {
    let theme = load_theme(None, None);
    terminal.draw(|frame| render_loading(frame, &theme))?;
    let entries = service.list_entries(workspace.root())?;
    let history = match history::load_entries(&workspace) {
        Ok(entries) => entries,
        Err(_) => Vec::new(),
    };
    let search_index = SearchIndex::new(workspace.search_db_path());
    search_index.start_background_rebuild(workspace.root().to_path_buf());
    let mut app = App::new(service, workspace, entries, history, search_index, theme);

    loop {
        if app.screen == Screen::Search {
            app.refresh_search_status();
        }
        app.poll_widget_load();
        let theme = app.theme.clone();
        terminal.draw(|frame| render_ui(frame, &mut app, &theme))?;

        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    handle_key_event(&mut app, key)
                }
                _ => {}
            }
        }

        if app.should_quit {
            return Ok(());
        }
        if let Some((script, args)) = app.result.take() {
            app.screen = Screen::Running;
            let theme = app.theme.clone();
            terminal.draw(|frame| render_ui(frame, &mut app, &theme))?;
            let run_result = service.run_script(&script, &args);
            let entry = match run_result {
                Ok(output) => history::success_entry(&app.workspace, &script, &args, output),
                Err(err) => history::error_entry(&app.workspace, &script, &args, err.to_string()),
            };
            let _ = history::record_entry(&app.workspace, &entry);
            app.add_history_entry(entry);
            app.back_to_script_select();
            app.reset_run_output_scroll();
            app.screen = Screen::RunResult;
        }
    }
}
