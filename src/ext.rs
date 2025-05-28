use crate::basic;
use serde_json::Value;

/// Working horse of the crate.
/// Implemented for `serde_json::Value`.
/// So you just place it in your scope and use the methods.
pub trait DataPathExt {
    /// Get shared reference to the value in `path` inside `self`
    fn path(&self, path: impl AsRef<str>) -> Option<&Self>;
    /// Get exclusive reference to the value in `path` inside `self`
    fn path_mut(&mut self, path: impl AsRef<str>) -> Option<&mut Self>;
    /// Update the `path` inside `self` with `value` or create the `path` if it does not exist and place `value` in it
    fn update_or_create(&mut self, path: impl AsRef<str>, value: Self) -> crate::Result<()>;
}

impl DataPathExt for Value {
    fn path(&self, path: impl AsRef<str>) -> Option<&Value> {
        basic::get(self, path)
    }

    fn path_mut(&mut self, path: impl AsRef<str>) -> Option<&mut Value> {
        basic::get_mut(self, path)
    }

    fn update_or_create(&mut self, path: impl AsRef<str>, value: Value) -> crate::Result<()> {
        basic::update_or_create(self, path, value)
    }
}
