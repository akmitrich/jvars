pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {
    // -- Externals
    Json(#[from] serde_json::Error),
    // -- Internal
    #[error("data path {0}")]
    InvalidDataPath(String),
    #[error("impossible")]
    Impossible(String),
}
