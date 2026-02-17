use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimelineError {
    #[error("event not found: {0}")]
    EventNotFound(String),
    #[error("tag not found: {0}")]
    TagNotFound(String),
    #[error("serialization error: {0}")]
    Postcard(#[from] postcard::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse datetime: {0}")]
    DateTimeParse(String),
    #[error("a range needs at least a --start or --end")]
    RangeMissingBound,
}

pub type TimelineResult<T> = Result<T, TimelineError>;
