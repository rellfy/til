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

    pub fn to_json(&self) -> TimelineResult<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_json(s: &str) -> TimelineResult<Self> {
        Ok(serde_json::from_str(s)?)
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

    #[test]
    fn to_json_produces_parseable_string() {
        let t = sample_timeline();
        let json = t.to_json().unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(v.get("id").is_some());
        assert!(v.get("events").is_some());
        assert!(v.get("ranges").is_some());
        assert!(v.get("tags").is_some());
        assert_eq!(v.get("label").and_then(|x| x.as_str()), Some("sample"));
    }

    #[test]
    fn json_roundtrip() {
        let original = sample_timeline();
        let json = original.to_json().unwrap();
        let parsed = Timeline::from_json(&json).unwrap();
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
    fn json_preserves_ref_and_attributes() {
        let original = sample_timeline();
        let json = original.to_json().unwrap();
        let parsed = Timeline::from_json(&json).unwrap();
        let original_event = original.events().values().next().unwrap();
        let parsed_event = parsed.events().get(original_event.id()).unwrap();
        assert_eq!(parsed_event.r#ref().as_deref(), Some("https://example.com"));
        assert_eq!(parsed_event.attributes().as_deref(), Some("{\"k\":1}"));
    }

    #[test]
    fn from_json_rejects_garbage() {
        assert!(Timeline::from_json("not json").is_err());
        assert!(Timeline::from_json("{}").is_err());
    }

    #[test]
    fn json_to_postcard_to_json_preserves_timeline() {
        let original = sample_timeline();
        let json1 = original.to_json().unwrap();
        let via_postcard_bytes = original.as_bytes().unwrap();
        let parsed_from_postcard = Timeline::from_bytes(&via_postcard_bytes).unwrap();
        let json2 = parsed_from_postcard.to_json().unwrap();
        let v1: serde_json::Value = serde_json::from_str(&json1).unwrap();
        let v2: serde_json::Value = serde_json::from_str(&json2).unwrap();
        assert_eq!(v1, v2);
    }
}
