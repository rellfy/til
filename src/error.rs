use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimelineError {
    #[error("event not found")]
    EventNotFound,
    #[error("serialization error: {0}")]
    Postcard(#[from] postcard::Error),
}

pub type TimelineResult<T> = Result<T, TimelineError>;
