use crate::search_index::{SearchDetails, SearchResult, SearchStatus};
use ratatui::widgets::ListState;

pub(crate) struct SearchState {
    pub(crate) query: String,
    pub(crate) results: Vec<SearchResult>,
    pub(crate) list_state: ListState,
    pub(crate) selection: usize,
    pub(crate) details: Option<SearchDetails>,
    pub(crate) status: SearchStatus,
    pub(crate) error: Option<String>,
}

impl SearchState {
    pub(crate) fn new(status: SearchStatus) -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
            list_state: ListState::default(),
            selection: 0,
            details: None,
            status,
            error: None,
        }
    }
}
