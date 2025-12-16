use crate::timeline::Timeline;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimelineError {
    #[error("event not found")]
    EventNotFound,
}

pub type TimelineResult<T> = Result<T, TimelineError>;

impl<T> Into<TimelineResult<T>> for TimelineError {
    fn into(self) -> TimelineResult<T> {
        Err(self)
    }
}
