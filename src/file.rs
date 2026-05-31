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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::{Event, EventRange, Range, Tag};
    use jiff::civil::DateTime;

    fn dt(s: &str) -> DateTime {
        s.parse().unwrap()
    }

    fn sample_timeline() -> Timeline {
        let mut t = Timeline::new("sample");
        let mut e = Event::new("first event", dt("2024-01-01T00:00:00"));
        e.set_ref(Some("https://example.com".to_string()));
        e.set_attributes(Some("{\"k\":1}".to_string()));
        t.add_event(e).unwrap();
        let r = Range::new(
            "first range",
            EventRange::StartEnd(dt("2024-01-01T00:00:00"), dt("2024-02-01T00:00:00")),
        );
        t.add_range(r).unwrap();
        t.add_tag(Tag::new("milestone")).unwrap();
        t
    }

    #[test]
    fn header_is_magic_plus_version() {
        let bytes = Timeline::new("x").as_bytes().unwrap();
        assert_eq!(&bytes[..4], b"TIL\0");
        assert_eq!(bytes[4], 1);
    }

    #[test]
    fn roundtrip_preserves_timeline() {
        let original = sample_timeline();
        let bytes = original.as_bytes().unwrap();
        let parsed = Timeline::from_bytes(&bytes).unwrap();
        assert_eq!(original.id(), parsed.id());
        assert_eq!(original.label(), parsed.label());
        assert_eq!(original.events().len(), parsed.events().len());
        assert_eq!(original.ranges().len(), parsed.ranges().len());
        assert_eq!(original.tags().len(), parsed.tags().len());
        for (id, ev) in original.events().iter() {
            assert_eq!(parsed.events().get(id), Some(ev));
        }
        for (id, r) in original.ranges().iter() {
            assert_eq!(parsed.ranges().get(id), Some(r));
        }
    }

    #[test]
    fn empty_timeline_roundtrips() {
        let bytes = Timeline::new("empty").as_bytes().unwrap();
        let parsed = Timeline::from_bytes(&bytes).unwrap();
        assert_eq!(parsed.events().len(), 0);
        assert_eq!(parsed.ranges().len(), 0);
        assert_eq!(parsed.tags().len(), 0);
    }

    #[test]
    fn bad_magic_rejected() {
        let bytes = vec![b'X', b'Y', b'Z', 0, 1];
        assert!(matches!(
            Timeline::from_bytes(&bytes).unwrap_err(),
            TimelineError::InvalidMagic,
        ));
    }

    #[test]
    fn short_input_rejected() {
        assert!(matches!(
            Timeline::from_bytes(&[]).unwrap_err(),
            TimelineError::InvalidMagic,
        ));
        assert!(matches!(
            Timeline::from_bytes(b"TIL").unwrap_err(),
            TimelineError::InvalidMagic,
        ));
    }

    #[test]
    fn future_version_rejected() {
        let mut bytes = b"TIL\0".to_vec();
        bytes.push(99);
        assert!(matches!(
            Timeline::from_bytes(&bytes).unwrap_err(),
            TimelineError::UnsupportedVersion(99),
        ));
    }
}
