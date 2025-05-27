use serde_json::Value;

use crate::basic;

pub trait DataPathExt {
    fn path(&self, path: impl AsRef<str>) -> Option<&Self>;
    fn path_mut(&mut self, path: impl AsRef<str>) -> Option<&mut Self>;
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
