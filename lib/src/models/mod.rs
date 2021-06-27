/// This file has mysteriously existed for so lang, and we never quite discussed about it
///
/// Here's we using the `habit` recognized by the module name as well as a matching filename relative to this file
mod habit;
mod habit_id;
mod habit_name;
mod habit_unit;
mod validation_error;
mod habit_draft;
mod habit_patch;
mod habit_log;

pub use habit::*;
pub use habit_id::*;
pub use habit_name::*;
pub use habit_unit::*;
pub use validation_error::*;
pub use habit_draft::*;
pub use habit_patch::*;
pub use habit_log::*;
