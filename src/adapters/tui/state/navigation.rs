use crate::domain::Schema;
use crate::lua_widget::WidgetData;
use crate::ports::WorkspaceEntry;
use ratatui::widgets::ListState;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use super::super::app::SchemaPreview;

#[derive(Debug)]
pub(crate) struct WidgetLoadResult {
    pub(crate) widget: Option<WidgetData>,
    pub(crate) error: Option<String>,
}

pub(crate) struct NavigationState {
    pub(crate) current_dir: PathBuf,
    pub(crate) entries: Vec<WorkspaceEntry>,
    pub(crate) list_state: ListState,
    pub(crate) selection: usize,
    pub(crate) widget: Option<WidgetData>,
    pub(crate) widget_error: Option<String>,
    pub(crate) widget_loading: bool,
    pub(crate) widget_receiver: Option<Receiver<WidgetLoadResult>>,
    pub(crate) schema_preview: Option<SchemaPreview>,
    pub(crate) schema_preview_error: Option<String>,
    pub(crate) preview_script: Option<PathBuf>,
    pub(crate) schema_cache: Option<(PathBuf, Schema)>,
}

impl NavigationState {
    pub(crate) fn new(current_dir: PathBuf, entries: Vec<WorkspaceEntry>) -> Self {
        let mut list_state = ListState::default();
        if !entries.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            current_dir,
            entries,
            list_state,
            selection: 0,
            widget: None,
            widget_error: None,
            widget_loading: false,
            widget_receiver: None,
            schema_preview: None,
            schema_preview_error: None,
            preview_script: None,
            schema_cache: None,
        }
    }
}
