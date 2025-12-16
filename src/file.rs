//! Code for composing and parsing the binary for a .til file.

use crate::error::TimelineResult;
use crate::timeline::Timeline;

impl Timeline {
    pub fn as_bytes(&self) -> Vec<u8> {}

    pub fn from_bytes(bytes: &[u8]) -> TimelineResult<Self> {
        todo!()
    }
}
