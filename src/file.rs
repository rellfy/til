use crate::error::{TimelineError, TimelineResult};
use crate::timeline::Timeline;

const MAGIC: &[u8; 4] = b"TIL\0";
const VERSION: u8 = 1;
const HEADER_LEN: usize = MAGIC.len() + 1;

impl Timeline {
    pub fn as_bytes(&self) -> TimelineResult<Vec<u8>> {
        let body = postcard::to_allocvec(self)?;
        let mut out = Vec::with_capacity(HEADER_LEN + body.len());
        out.extend_from_slice(MAGIC);
        out.push(VERSION);
        out.extend_from_slice(&body);
        Ok(out)
    }

    pub fn from_bytes(bytes: &[u8]) -> TimelineResult<Self> {
        if bytes.len() < HEADER_LEN || &bytes[..MAGIC.len()] != MAGIC {
            return Err(TimelineError::InvalidMagic);
        }
        let version = bytes[MAGIC.len()];
        if version != VERSION {
            return Err(TimelineError::UnsupportedVersion(version));
        }
        Ok(postcard::from_bytes(&bytes[HEADER_LEN..])?)
    }
}
