use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Api error: {0}")]
    Api(String),

    #[error("placing limit order requires price")]
    PlacingLimitOrderRequiresPrice,

    #[error("endpoint requires auth but no secret configured")]
    NoSecretConfigured,

    #[error(transparent)]
    SerdeQs(#[from] serde_qs::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
