use crate::adapters::environments::FsEnvironmentRepository;
use crate::domain::Schema;
use crate::history::HistoryEntry;
use crate::lua_widget::{self, WidgetData};
use crate::ports::{WorkspaceEntry, WorkspaceEntryKind};
use crate::search_index::SearchIndex;
use crate::use_cases::{EnvironmentService, ScriptService};
use crate::workspace::Workspace;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, TryRecvError};

pub(crate) use super::state::HistoryFocus;
use super::state::{
    EnvironmentState, FieldInputState, HistoryState, NavigationState, SearchState, WidgetLoadResult,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Screen {
    ScriptSelect,
    Search,
    Environments,
    FieldInput,
    History,
    Running,
    RunResult,
    Error,
}

#[derive(Debug, Clone)]
pub(crate) struct SchemaPreview {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) fields: Vec<SchemaFieldPreview>,
    pub(crate) outputs: Vec<SchemaOutputPreview>,
    pub(crate) queue: Option<QueuePreview>,
}

#[derive(Debug, Clone)]
pub(crate) struct SchemaOutputPreview {
    pub(crate) name: String,
    pub(crate) kind: String,
}

#[derive(Debug, Clone)]
pub(crate) enum QueuePreview {
    Matrix { values: Vec<MatrixPreview> },
    Cases { cases: Vec<QueueCasePreview> },
}

#[derive(Debug, Clone)]
pub(crate) struct MatrixPreview {
    pub(crate) name: String,
    pub(crate) values: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct QueueCasePreview {
    pub(crate) name: Option<String>,
    pub(crate) values: Vec<QueueCaseValuePreview>,
}

#[derive(Debug, Clone)]
pub(crate) struct QueueCaseValuePreview {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Debug, Clone)]
pub(crate) struct SchemaFieldPreview {
    pub(crate) name: String,
    pub(crate) prompt: Option<String>,
    pub(crate) kind: String,
    pub(crate) required: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum ExecutionStatus {
    Success,
    Failed(Option<i32>),
    Error,
}

pub(crate) struct App<'a> {
    service: &'a ScriptService,
    pub(crate) workspace: Workspace,
    pub(crate) screen: Screen,
    env_return: Option<Screen>,
    search_index: SearchIndex,
    pub(crate) navigation: NavigationState,
    pub(crate) environment: EnvironmentState,
    pub(crate) search: SearchState,
    pub(crate) history: HistoryState,
    pub(crate) field_input: FieldInputState,
    pub(crate) result: Option<(PathBuf, Vec<String>)>,
    pub(crate) should_quit: bool,
    pub(crate) run_output_scroll: u16,
    pub(crate) error_message: Option<String>,
}

impl<'a> App<'a> {
    pub(crate) fn new(
        service: &'a ScriptService,
        workspace: Workspace,
        entries: Vec<WorkspaceEntry>,
        history: Vec<HistoryEntry>,
        search_index: SearchIndex,
    ) -> Self {
        let current_dir = workspace.root().to_path_buf();
        let navigation = NavigationState::new(current_dir, entries);
        let history = HistoryState::new(history);
        let search_status = search_index.status();
        let search = SearchState::new(search_status);
        let environment = EnvironmentState::new();
        let field_input = FieldInputState::new();
        let mut app = Self {
            service,
            workspace,
            screen: Screen::ScriptSelect,
            env_return: None,
            search_index,
            navigation,
            environment,
            search,
            history,
            field_input,
            result: None,
            should_quit: false,
            run_output_scroll: 0,
            error_message: None,
        };
        app.start_widget_load();
        app.load_env_config();
        app.update_schema_preview();
        app.update_env_preview();
        app
    }

    pub(crate) fn selected_entry(&self) -> Option<&WorkspaceEntry> {
        self.navigation.entries.get(self.navigation.selection)
    }

    pub(crate) fn move_selection(&mut self, delta: isize) {
        if self.navigation.entries.is_empty() {
            return;
        }
        let len = self.navigation.entries.len() as isize;
        let mut new_index = self.navigation.selection as isize + delta;
        if new_index < 0 {
            new_index = 0;
        } else if new_index >= len {
            new_index = len - 1;
        }
        self.navigation.selection = new_index as usize;
        self.navigation
            .list_state
            .select(Some(self.navigation.selection));
        self.update_schema_preview();
    }

    pub(crate) fn enter_search(&mut self) {
        self.search.status = self.search_index.status();
        self.screen = Screen::Search;
        self.refresh_search_results();
    }

    pub(crate) fn enter_envs(&mut self) {
        self.env_return = Some(self.screen);
        self.load_env_config();
        self.update_env_preview();
        self.screen = Screen::Environments;
    }

    pub(crate) fn exit_envs(&mut self) {
        self.screen = self.env_return.unwrap_or(Screen::ScriptSelect);
        self.env_return = None;
    }

    pub(crate) fn scroll_env_preview(&mut self, delta: i16) {
        let mut next = self.environment.preview_scroll as i16 + delta;
        if next < 0 {
            next = 0;
        }
        if next > u16::MAX as i16 {
            next = u16::MAX as i16;
        }
        self.environment.preview_scroll = next as u16;
    }

    pub(crate) fn move_env_selection(&mut self, delta: isize) {
        if self.environment.entries.is_empty() {
            return;
        }
        let len = self.environment.entries.len() as isize;
        let mut new_index = self.environment.selection as isize + delta;
        if new_index < 0 {
            new_index = 0;
        } else if new_index >= len {
            new_index = len - 1;
        }
        self.environment.selection = new_index as usize;
        self.environment
            .list_state
            .select(Some(self.environment.selection));
        self.update_env_preview();
    }

    pub(crate) fn activate_selected_env(&mut self) {
        if self.environment.entries.is_empty() {
            return;
        }
        let name = self.environment.entries[self.environment.selection]
            .name
            .clone();
        let service = self.environment_service();
        match service.set_active_env(Some(&name)) {
            Ok(()) => self.load_env_config(),
            Err(err) => self.environment.error = Some(err.to_string()),
        }
    }

    pub(crate) fn deactivate_env(&mut self) {
        let service = self.environment_service();
        match service.set_active_env(None) {
            Ok(()) => self.load_env_config(),
            Err(err) => self.environment.error = Some(err.to_string()),
        }
    }

    pub(crate) fn refresh_search_status(&mut self) {
        let status = self.search_index.status();
        if status != self.search.status {
            self.search.status = status.clone();
            if self.screen == Screen::Search {
                self.refresh_search_results();
            }
        }
    }

    pub(crate) fn move_search_selection(&mut self, delta: isize) {
        if self.search.results.is_empty() {
            return;
        }
        let len = self.search.results.len() as isize;
        let mut new_index = self.search.selection as isize + delta;
        if new_index < 0 {
            new_index = 0;
        } else if new_index >= len {
            new_index = len - 1;
        }
        self.search.selection = new_index as usize;
        self.search.list_state.select(Some(self.search.selection));
        self.update_search_details();
    }

    pub(crate) fn append_search_char(&mut self, ch: char) {
        self.search.query.push(ch);
        self.refresh_search_results();
    }

    pub(crate) fn pop_search_char(&mut self) {
        self.search.query.pop();
        self.refresh_search_results();
    }

    pub(crate) fn open_selected_search(&mut self) {
        let entry = match self.search.results.get(self.search.selection) {
            Some(entry) => entry,
            None => return,
        };
        let script_path = self.workspace.root().join(&entry.script_path);
        self.load_schema(script_path);
    }

    pub(crate) fn enter_selected(&mut self) {
        let entry = match self.selected_entry() {
            Some(entry) => entry.clone(),
            None => return,
        };

        match entry.kind {
            WorkspaceEntryKind::Directory => {
                self.navigation.current_dir = entry.path;
                self.refresh_entries();
            }
            WorkspaceEntryKind::Script => {
                self.load_schema(entry.path);
            }
        }
    }

    pub(crate) fn navigate_up(&mut self) {
        if self.navigation.current_dir == self.workspace.root() {
            return;
        }
        if let Some(parent) = self.navigation.current_dir.parent() {
            self.navigation.current_dir = parent.to_path_buf();
            self.refresh_entries();
        }
    }

    pub(crate) fn move_history_selection(&mut self, delta: isize) {
        if self.history.entries.is_empty() {
            return;
        }
        let len = self.history.entries.len() as isize;
        let mut new_index = self.history.selection as isize + delta;
        if new_index < 0 {
            new_index = 0;
        } else if new_index >= len {
            new_index = len - 1;
        }
        self.history.selection = new_index as usize;
        self.history
            .table_state
            .select(Some(self.history.selection));
        self.reset_run_output_scroll();
    }

    pub(crate) fn add_history_entry(&mut self, entry: HistoryEntry) {
        self.history.entries.insert(0, entry);
        self.history.selection = 0;
        self.history.table_state.select(Some(0));
    }

    pub(crate) fn current_history_entry(&self) -> Option<&HistoryEntry> {
        self.history.entries.get(self.history.selection)
    }

    pub(crate) fn load_schema(&mut self, script: PathBuf) {
        let schema_result = match self.navigation.schema_cache.as_ref() {
            Some((path, schema)) if path == &script => Ok(schema.clone()),
            _ => self.service.load_schema(&script),
        };

        match schema_result {
            Ok(mut schema) => {
                self.load_env_config();
                schema.fields.sort_by_key(|field| field.order);
                let tags = schema.tags.clone();
                let outputs = schema.outputs.clone();
                let queue = schema.queue.clone();
                self.field_input.schema_name = Some(schema.name);
                self.field_input.schema_description = schema.description;
                self.field_input.fields = schema.fields;
                self.field_input.field_index = 0;
                self.field_input.field_inputs = self.build_field_inputs();
                self.field_input.args.clear();
                self.field_input.error = None;
                self.field_input.selected_script = Some(script.clone());
                self.navigation.schema_cache = Some((
                    script.clone(),
                    Schema {
                        name: self.field_input.schema_name.clone().unwrap_or_default(),
                        description: self.field_input.schema_description.clone(),
                        tags,
                        fields: self.field_input.fields.clone(),
                        outputs,
                        queue,
                    },
                ));
                if self.field_input.fields.is_empty() {
                    self.result = Some((script, Vec::new()));
                } else {
                    self.screen = Screen::FieldInput;
                }
            }
            Err(err) => {
                self.error_message = Some(err.to_string());
                self.screen = Screen::Error;
            }
        }
    }

    pub(crate) fn move_field_selection(&mut self, delta: isize) {
        if self.field_input.fields.is_empty() {
            return;
        }
        let len = self.field_input.fields.len() as isize;
        let mut new_index = self.field_input.field_index as isize + delta;
        while new_index < 0 {
            new_index += len;
        }
        while new_index >= len {
            new_index -= len;
        }
        self.field_input.field_index = new_index as usize;
        self.field_input.error = None;
    }

    pub(crate) fn append_field_char(&mut self, ch: char) {
        if let Some(value) = self
            .field_input
            .field_inputs
            .get_mut(self.field_input.field_index)
        {
            value.push(ch);
            self.field_input.error = None;
        }
    }

    pub(crate) fn pop_field_char(&mut self) {
        if let Some(value) = self
            .field_input
            .field_inputs
            .get_mut(self.field_input.field_index)
        {
            value.pop();
            self.field_input.error = None;
        }
    }

    pub(crate) fn submit_form(&mut self) {
        if self.field_input.fields.is_empty() {
            self.finish();
            return;
        }

        let mut args = Vec::new();
        for (idx, field) in self.field_input.fields.iter().enumerate() {
            let input = self
                .field_input
                .field_inputs
                .get(idx)
                .map(String::as_str)
                .unwrap_or("");
            match crate::domain::normalize_input(field, input) {
                Ok(value) => {
                    if let Some(value) = value {
                        let arg = field
                            .arg
                            .clone()
                            .unwrap_or_else(|| format!("--{}", field.name));
                        args.push(arg);
                        args.push(value);
                    }
                }
                Err(message) => {
                    self.field_input.error = Some(format!("{}: {}", field.name, message));
                    self.field_input.field_index = idx;
                    return;
                }
            }
        }

        self.field_input.args = args;
        self.field_input.error = None;
        self.finish();
    }

    fn finish(&mut self) {
        if let Some(script) = &self.field_input.selected_script {
            self.result = Some((script.clone(), self.field_input.args.clone()));
        } else {
            self.should_quit = true;
        }
    }

    pub(crate) fn refresh_entries(&mut self) {
        match self.service.list_entries(&self.navigation.current_dir) {
            Ok(entries) => {
                self.navigation.entries = entries;
                self.navigation.selection = 0;
                if self.navigation.entries.is_empty() {
                    self.navigation.list_state.select(None);
                } else {
                    self.navigation.list_state.select(Some(0));
                }
                self.error_message = None;
                self.start_widget_load();
                self.update_schema_preview();
            }
            Err(err) => {
                self.error_message = Some(err.to_string());
                self.screen = Screen::Error;
            }
        }
    }

    pub(crate) fn refresh_status(&mut self) {
        self.start_widget_load();
        self.load_env_config();
        self.update_schema_preview();
    }

    pub(crate) fn back_to_script_select(&mut self) {
        self.screen = Screen::ScriptSelect;
        self.field_input.schema_name = None;
        self.field_input.schema_description = None;
        self.field_input.fields.clear();
        self.field_input.field_index = 0;
        self.field_input.field_inputs.clear();
        self.field_input.args.clear();
        self.field_input.error = None;
        self.field_input.selected_script = None;
        self.result = None;
    }

    pub(crate) fn reset_run_output_scroll(&mut self) {
        self.run_output_scroll = 0;
    }

    pub(crate) fn scroll_run_output(&mut self, delta: i16) {
        if delta > 0 {
            self.run_output_scroll = self.run_output_scroll.saturating_add(delta as u16);
        } else if delta < 0 {
            let amount = (-delta) as u16;
            self.run_output_scroll = self.run_output_scroll.saturating_sub(amount);
        }
    }

    pub(crate) fn display_path(&self, path: &Path) -> String {
        path.strip_prefix(self.workspace.root())
            .unwrap_or(path)
            .to_string_lossy()
            .to_string()
    }

    fn start_widget_load(&mut self) {
        let dir = self.navigation.current_dir.clone();
        let (tx, rx) = mpsc::channel();
        self.navigation.widget_loading = true;
        self.navigation.widget = None;
        self.navigation.widget_error = None;
        self.navigation.widget_receiver = Some(rx);
        std::thread::spawn(move || {
            let (widget, error) = load_widget_state(&dir);
            let _ = tx.send(WidgetLoadResult { widget, error });
        });
    }

    pub(crate) fn poll_widget_load(&mut self) {
        let Some(receiver) = &self.navigation.widget_receiver else {
            return;
        };

        match receiver.try_recv() {
            Ok(result) => {
                self.navigation.widget = result.widget;
                self.navigation.widget_error = result.error;
                self.navigation.widget_loading = false;
                self.navigation.widget_receiver = None;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                self.navigation.widget_loading = false;
                self.navigation.widget_receiver = None;
            }
        }
    }

    fn environment_service(&self) -> EnvironmentService {
        let repo = FsEnvironmentRepository::new(self.workspace.envs_dir());
        EnvironmentService::new(Box::new(repo))
    }

    fn load_env_config(&mut self) {
        let mut env_error = None;

        let service = self.environment_service();
        let env_config = match service.load_environment_config() {
            Ok(config) => Some(config),
            Err(err) => {
                env_error = Some(err.to_string());
                None
            }
        };

        let env_entries = match service.list_env_files() {
            Ok(entries) => entries,
            Err(err) => {
                if env_error.is_none() {
                    env_error = Some(err.to_string());
                }
                Vec::new()
            }
        };

        let selected = if env_entries.is_empty() {
            0
        } else if let Some(active) = env_config
            .as_ref()
            .and_then(|config| config.active.as_ref())
        {
            env_entries
                .iter()
                .position(|entry| entry.name == *active)
                .unwrap_or(0)
        } else {
            self.environment
                .selection
                .min(env_entries.len().saturating_sub(1))
        };

        self.environment.entries = env_entries;
        self.environment.selection = selected;
        if self.environment.entries.is_empty() {
            self.environment.list_state.select(None);
        } else {
            self.environment
                .list_state
                .select(Some(self.environment.selection));
        }

        self.environment.config = env_config;
        self.environment.error = env_error;
        self.update_env_preview();
    }

    fn update_env_preview(&mut self) {
        self.environment.preview_scroll = 0;
        self.environment.preview_error = None;

        let entry = match self.environment.entries.get(self.environment.selection) {
            Some(entry) => entry,
            None => {
                self.environment.preview_lines = Vec::new();
                return;
            }
        };

        let envs_dir = self
            .environment
            .config
            .as_ref()
            .map(|config| config.envs_dir.clone())
            .unwrap_or_else(|| self.workspace.envs_dir().to_path_buf());
        let env_path = envs_dir.join(&entry.name);

        let service = self.environment_service();
        match service.load_env_preview(&env_path) {
            Ok(entries) => {
                let mut lines = Vec::new();
                for (key, value) in entries {
                    let line = ratatui::text::Line::from(vec![
                        ratatui::text::Span::styled(
                            key,
                            ratatui::style::Style::default()
                                .fg(ratatui::style::Color::Yellow)
                                .add_modifier(ratatui::style::Modifier::BOLD),
                        ),
                        ratatui::text::Span::styled(
                            " = ",
                            ratatui::style::Style::default().fg(ratatui::style::Color::Gray),
                        ),
                        ratatui::text::Span::raw(value),
                    ]);
                    lines.push(line);
                }
                if lines.is_empty() {
                    self.environment.preview_lines =
                        vec![ratatui::text::Line::from(ratatui::text::Span::styled(
                            "No entries found.",
                            ratatui::style::Style::default().fg(ratatui::style::Color::Gray),
                        ))];
                } else {
                    self.environment.preview_lines = lines;
                }
                self.environment.preview_error = None;
            }
            Err(err) => {
                self.environment.preview_lines = Vec::new();
                self.environment.preview_error = Some(err.to_string());
            }
        }
    }

    fn build_field_inputs(&self) -> Vec<String> {
        let defaults = self
            .environment
            .config
            .as_ref()
            .map(|config| &config.defaults);
        match defaults {
            Some(defaults) if !defaults.is_empty() => self
                .field_input
                .fields
                .iter()
                .map(|field| {
                    defaults
                        .get(&field.name.to_ascii_lowercase())
                        .cloned()
                        .unwrap_or_default()
                })
                .collect(),
            _ => vec![String::new(); self.field_input.fields.len()],
        }
    }

    fn update_schema_preview(&mut self) {
        let (entry_path, entry_kind) = match self.selected_entry() {
            Some(entry) => (entry.path.clone(), entry.kind),
            None => {
                self.navigation.schema_preview = None;
                self.navigation.schema_preview_error = None;
                self.navigation.preview_script = None;
                return;
            }
        };

        if entry_kind != WorkspaceEntryKind::Script {
            self.navigation.schema_preview = None;
            self.navigation.schema_preview_error = None;
            self.navigation.preview_script = None;
            return;
        }

        if self.navigation.preview_script.as_ref() == Some(&entry_path) {
            return;
        }

        match self.service.load_schema(&entry_path) {
            Ok(mut schema) => {
                schema.fields.sort_by_key(|field| field.order);
                self.navigation.schema_preview = Some(schema_to_preview(&schema));
                self.navigation.schema_preview_error = None;
                self.navigation.preview_script = Some(entry_path.clone());
                self.navigation.schema_cache = Some((entry_path, schema));
            }
            Err(err) => {
                self.navigation.schema_preview = None;
                self.navigation.schema_preview_error = Some(err.to_string());
                self.navigation.preview_script = Some(entry_path);
            }
        }
    }

    fn refresh_search_results(&mut self) {
        match self.search_index.query(&self.search.query) {
            Ok(results) => {
                self.search.results = results;
                self.search.error = None;
            }
            Err(err) => {
                self.search.results.clear();
                self.search.error = Some(err);
            }
        }
        self.search.selection = 0;
        if self.search.results.is_empty() {
            self.search.list_state.select(None);
        } else {
            self.search.list_state.select(Some(0));
        }
        self.update_search_details();
    }

    fn update_search_details(&mut self) {
        self.search.details = None;
        let entry = match self.search.results.get(self.search.selection) {
            Some(entry) => entry,
            None => return,
        };
        match self.search_index.load_details(&entry.script_path) {
            Ok(details) => {
                self.search.details = details;
                self.search.error = None;
            }
            Err(err) => {
                self.search.error = Some(err);
            }
        }
    }
}

impl ExecutionStatus {
    pub(crate) fn from_history(entry: &HistoryEntry) -> Self {
        if entry.error.is_some() {
            ExecutionStatus::Error
        } else if entry.success {
            ExecutionStatus::Success
        } else {
            ExecutionStatus::Failed(entry.exit_code)
        }
    }
}

fn load_widget_state(dir: &Path) -> (Option<WidgetData>, Option<String>) {
    match lua_widget::load_widget(dir) {
        Ok(widget) => (widget, None),
        Err(err) => (None, Some(err)),
    }
}

fn schema_to_preview(schema: &Schema) -> SchemaPreview {
    let tags = schema.tags.clone().unwrap_or_default();
    let fields = schema
        .fields
        .iter()
        .map(|field| SchemaFieldPreview {
            name: field.name.clone(),
            prompt: field.prompt.clone(),
            kind: field.kind.clone(),
            required: field.required.unwrap_or(false),
        })
        .collect();
    let outputs = schema
        .outputs
        .as_ref()
        .map(|items| {
            items
                .iter()
                .map(|output| SchemaOutputPreview {
                    name: output.name.clone(),
                    kind: output.kind.clone(),
                })
                .collect()
        })
        .unwrap_or_default();

    let queue = schema.queue.as_ref().map(|queue| {
        if let Some(matrix) = &queue.matrix {
            QueuePreview::Matrix {
                values: matrix
                    .values
                    .iter()
                    .map(|value| MatrixPreview {
                        name: value.name.clone(),
                        values: value.values.clone(),
                    })
                    .collect(),
            }
        } else if let Some(cases) = &queue.cases {
            QueuePreview::Cases {
                cases: cases
                    .iter()
                    .map(|case| QueueCasePreview {
                        name: case.name.clone(),
                        values: case
                            .values
                            .iter()
                            .map(|value| QueueCaseValuePreview {
                                name: value.name.clone(),
                                value: value.value.clone(),
                            })
                            .collect(),
                    })
                    .collect(),
            }
        } else {
            QueuePreview::Cases { cases: Vec::new() }
        }
    });

    SchemaPreview {
        name: schema.name.clone(),
        description: schema.description.clone(),
        tags,
        fields,
        outputs,
        queue,
    }
}
