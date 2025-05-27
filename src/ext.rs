use serde_json::Value;

use crate::basic;

pub trait DataPathExt {
    fn update_or_create(&mut self, path: impl AsRef<str>, value: Self) -> crate::Result<()>;
}

impl DataPathExt for Value {
    fn update_or_create(&mut self, path: impl AsRef<str>, value: Value) -> crate::Result<()> {
        basic::update_or_create(self, path, value)
    }
}
