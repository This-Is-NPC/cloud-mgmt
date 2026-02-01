//! Domain layer - core types and validation logic.

mod parsing;
mod schema;
mod validation;

pub use parsing::{extract_schema_block, parse_schema};
#[allow(unused_imports)]
pub use schema::{
    CaseValue, Field, MatrixSpec, MatrixValue, OutputField, QueueCase, QueueSpec, Schema,
};
pub use validation::normalize_input;
