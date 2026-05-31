use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimelineError {
    #[error("event not found: {0}")]
    EventNotFound(String),
    #[error("range not found: {0}")]
    RangeNotFound(String),
    #[error("tag not found: {0}")]
    TagNotFound(String),
    #[error("event already exists: {0}")]
    EventLabelExists(String),
    #[error("range already exists: {0}")]
    RangeLabelExists(String),
    #[error("tag already exists: {0}")]
    TagLabelExists(String),
    #[error("serialization error: {0}")]
    Postcard(#[from] postcard::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse datetime: {0}")]
    DateTimeParse(String),
    #[error("a range needs at least a --start or --end")]
    RangeMissingBound,
    #[error("invalid file path: {0}")]
    InvalidPath(String),
    #[error("not a .til file (bad magic)")]
    InvalidMagic,
    #[error("unsupported .til format version: {0}")]
    UnsupportedVersion(u8),
    #[error("failed to parse attributes JSON: {0}")]
    AttributesParse(String),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("selector ambiguous: pass either a label or --id, not both")]
    SelectorAmbiguous,
    #[error("selector missing: pass either a label or --id")]
    SelectorMissing,
}

pub type TimelineResult<T> = Result<T, TimelineError>;
