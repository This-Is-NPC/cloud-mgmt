//! Domain layer - core types and validation logic.

mod parsing;
mod schema;
mod validation;

pub use parsing::{extract_schema_block, parse_schema};
pub use schema::{Field, Schema};
pub use validation::normalize_input;
