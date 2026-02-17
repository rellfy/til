use crate::error::TimelineResult;
use crate::timeline::Timeline;

impl Timeline {
    pub fn as_bytes(&self) -> TimelineResult<Vec<u8>> {
        Ok(postcard::to_allocvec(self)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> TimelineResult<Self> {
        Ok(postcard::from_bytes(bytes)?)
    }
}
