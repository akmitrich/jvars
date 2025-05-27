mod basic;
mod error;

pub use basic::{get, get_mut, update_or_create};
pub use error::{Error, Result};
