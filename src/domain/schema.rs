use serde::Deserialize;

/// Schema definition for a script.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Schema {
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub fields: Vec<Field>,
    pub outputs: Option<Vec<OutputField>>,
    pub queue: Option<QueueSpec>,
}

/// Script input field definition.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Field {
    pub name: String,
    pub prompt: Option<String>,
    #[serde(rename = "Type")]
    pub kind: String,
    pub order: u32,
    pub required: Option<bool>,
    pub default: Option<String>,
    pub choices: Option<Vec<String>>,
    pub arg: Option<String>,
}

/// Script output field definition.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct OutputField {
    pub name: String,
    #[serde(rename = "Type")]
    pub kind: String,
}

/// Optional queue specification for batch execution.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct QueueSpec {
    pub matrix: Option<MatrixSpec>,
    pub cases: Option<Vec<QueueCase>>,
}

/// Matrix specification for batch execution.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MatrixSpec {
    pub values: Vec<MatrixValue>,
}

/// Matrix value.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MatrixValue {
    pub name: String,
    pub values: Vec<String>,
}

/// Queue case entry.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct QueueCase {
    pub name: Option<String>,
    pub values: Vec<CaseValue>,
}

/// Queue case value.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CaseValue {
    pub name: String,
    pub value: String,
}
