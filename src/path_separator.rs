use std::sync::RwLock;

pub struct PathSeparator {
    inner: RwLock<&'static str>,
}

impl PathSeparator {
    const fn new() -> Self {
        Self {
            inner: RwLock::new("."),
        }
    }

    /// Change the path separator used in basic JSON operations
    /// Best way to use this feature is to change the separator only once at the start up of your application
    pub fn change(&self, path_separator: &'static str) {
        *self.inner.write().unwrap() = path_separator;
    }

    pub(crate) fn get(&self) -> &'static str {
        *self.inner.read().unwrap()
    }
}

pub static PATH_SEPARATOR: PathSeparator = PathSeparator::new();
