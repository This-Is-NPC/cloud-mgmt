use crate::domain::Field;
use std::path::PathBuf;

pub(crate) struct FieldInputState {
    pub(crate) schema_name: Option<String>,
    pub(crate) schema_description: Option<String>,
    pub(crate) fields: Vec<Field>,
    pub(crate) field_index: usize,
    pub(crate) field_inputs: Vec<String>,
    pub(crate) args: Vec<String>,
    pub(crate) error: Option<String>,
    pub(crate) selected_script: Option<PathBuf>,
}

impl FieldInputState {
    pub(crate) fn new() -> Self {
        Self {
            schema_name: None,
            schema_description: None,
            fields: Vec::new(),
            field_index: 0,
            field_inputs: Vec::new(),
            args: Vec::new(),
            error: None,
            selected_script: None,
        }
    }
}
