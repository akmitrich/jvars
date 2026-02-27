use crate::basic;
use serde_json::Value;

/// Working horse of the crate.
/// Implemented for `serde_json::Value`.
/// So you just place it in your scope and use the methods.
pub trait DataPathExt {
    type Value;

    /// Get shared reference to the value in `path` inside `self`
    fn path<A: AsRef<str>>(&self, path: impl IntoIterator<Item = A>) -> Option<&Self::Value>;
    /// Get exclusive reference to the value in `path` inside `self`
    fn path_mut<A: AsRef<str>>(
        &mut self,
        path: impl IntoIterator<Item = A>,
    ) -> Option<&mut Self::Value>;
    /// Update the `path` inside `self` with `value` or create the `path` if it does not exist and place `value` in it
    fn update_or_create(&mut self, path: impl AsRef<str>, value: Self::Value) -> crate::Result<()>;
    /// Delete the value in the `path` and return it; returns None if there is no value in the `path`
    fn delete(&mut self, path: impl AsRef<str>) -> Option<Self::Value>;
}

impl DataPathExt for Value {
    type Value = Self;

    fn path<A: AsRef<str>>(&self, path: impl IntoIterator<Item = A>) -> Option<&Self::Value> {
        path.into_iter().try_fold(self, |sub_value, next| {
            let next = next.as_ref();
            match sub_value {
                Value::Array(a) => a.get(next.parse::<usize>().ok()?),
                _ => sub_value.get(next),
            }
        })
    }

    fn path_mut<A: AsRef<str>>(
        &mut self,
        path: impl IntoIterator<Item = A>,
    ) -> Option<&mut Self::Value> {
        path.into_iter().try_fold(self, |sub_value, next| {
            let next = next.as_ref();
            match sub_value {
                Value::Array(a) => a.get_mut(next.parse::<usize>().ok()?),
                sub_value => sub_value.get_mut(next),
            }
        })
    }

    fn update_or_create(&mut self, path: impl AsRef<str>, value: Self::Value) -> crate::Result<()> {
        basic::update_or_create(self, path, value)
    }

    fn delete(&mut self, path: impl AsRef<str>) -> Option<Self::Value> {
        basic::delete(self, path)
    }
}
