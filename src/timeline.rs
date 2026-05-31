use crate::error::{TimelineError, TimelineResult};
use crate::unit::{Event, Range, Tag};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Getters, Serialize, Deserialize)]
pub struct Timeline {
    id: Uuid,
    events: HashMap<Uuid, Event>,
    ranges: HashMap<Uuid, Range>,
    tags: HashMap<Uuid, Tag>,
    label: String,
}

impl Timeline {
    pub fn new(label: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            events: HashMap::new(),
            ranges: HashMap::new(),
            tags: HashMap::new(),
            label: label.to_string(),
        }
    }

    pub fn add_event(&mut self, event: Event) -> TimelineResult<()> {
        if self.events.values().any(|e| e.label() == event.label()) {
            return Err(TimelineError::EventLabelExists(event.label().clone()));
        }
        self.events.insert(*event.id(), event);
        Ok(())
    }

    pub fn remove_event(&mut self, label: &str) -> TimelineResult<()> {
        let id = self
            .events
            .values()
            .find(|e| e.label() == label)
            .map(|e| *e.id())
            .ok_or_else(|| TimelineError::EventNotFound(label.to_string()))?;
        self.events.remove(&id);
        Ok(())
    }

    pub fn tag_event(&mut self, tag_label: &str, event_label: &str) -> TimelineResult<()> {
        let tag_id = self.find_or_create_tag(tag_label);
        let event = self
            .events
            .values_mut()
            .find(|e| e.label() == event_label)
            .ok_or_else(|| TimelineError::EventNotFound(event_label.to_string()))?;
        event.add_tag(tag_id);
        Ok(())
    }

    pub fn untag_event(&mut self, tag_label: &str, event_label: &str) -> TimelineResult<()> {
        let tag_id = self
            .tags
            .values()
            .find(|t| t.label() == tag_label)
            .map(|t| *t.id())
            .ok_or_else(|| TimelineError::TagNotFound(tag_label.to_string()))?;
        let event = self
            .events
            .values_mut()
            .find(|e| e.label() == event_label)
            .ok_or_else(|| TimelineError::EventNotFound(event_label.to_string()))?;
        event.remove_tag(&tag_id);
        Ok(())
    }

    pub fn add_tag(&mut self, tag: Tag) -> TimelineResult<()> {
        if self.tags.values().any(|t| t.label() == tag.label()) {
            return Err(TimelineError::TagLabelExists(tag.label().clone()));
        }
        self.tags.insert(*tag.id(), tag);
        Ok(())
    }

    pub fn delete_tag(&mut self, label: &str) -> TimelineResult<()> {
        let tag_id = self
            .tags
            .values()
            .find(|t| t.label() == label)
            .map(|t| *t.id())
            .ok_or_else(|| TimelineError::TagNotFound(label.to_string()))?;
        self.tags.remove(&tag_id);
        for event in self.events.values_mut() {
            event.remove_tag(&tag_id);
        }
        for range in self.ranges.values_mut() {
            range.remove_tag(&tag_id);
        }
        Ok(())
    }

    pub fn tag_range(&mut self, tag_label: &str, range_label: &str) -> TimelineResult<()> {
        let tag_id = self.find_or_create_tag(tag_label);
        let range = self
            .ranges
            .values_mut()
            .find(|r| r.label() == range_label)
            .ok_or_else(|| TimelineError::RangeNotFound(range_label.to_string()))?;
        range.add_tag(tag_id);
        Ok(())
    }

    pub fn untag_range(&mut self, tag_label: &str, range_label: &str) -> TimelineResult<()> {
        let tag_id = self
            .tags
            .values()
            .find(|t| t.label() == tag_label)
            .map(|t| *t.id())
            .ok_or_else(|| TimelineError::TagNotFound(tag_label.to_string()))?;
        let range = self
            .ranges
            .values_mut()
            .find(|r| r.label() == range_label)
            .ok_or_else(|| TimelineError::RangeNotFound(range_label.to_string()))?;
        range.remove_tag(&tag_id);
        Ok(())
    }

    pub fn add_range(&mut self, range: Range) -> TimelineResult<()> {
        if self.ranges.values().any(|r| r.label() == range.label()) {
            return Err(TimelineError::RangeLabelExists(range.label().clone()));
        }
        self.ranges.insert(*range.id(), range);
        Ok(())
    }

    pub fn remove_range(&mut self, label: &str) {
        self.ranges.retain(|_, r| r.label() != label);
    }

    pub fn merge(&mut self, other: Timeline) -> TimelineResult<()> {
        for tag in other.tags.values() {
            if self.tags.values().any(|t| t.label() == tag.label()) {
                return Err(TimelineError::TagLabelExists(tag.label().clone()));
            }
        }
        for event in other.events.values() {
            if self.events.values().any(|e| e.label() == event.label()) {
                return Err(TimelineError::EventLabelExists(event.label().clone()));
            }
        }
        for range in other.ranges.values() {
            if self.ranges.values().any(|r| r.label() == range.label()) {
                return Err(TimelineError::RangeLabelExists(range.label().clone()));
            }
        }
        self.tags.extend(other.tags);
        self.events.extend(other.events);
        self.ranges.extend(other.ranges);
        Ok(())
    }

    pub fn tag_label(&self, tag_id: &Uuid) -> Option<&str> {
        self.tags.get(tag_id).map(|t| t.label().as_str())
    }

    fn find_or_create_tag(&mut self, label: &str) -> Uuid {
        if let Some(tag) = self.tags.values().find(|t| t.label() == label) {
            return *tag.id();
        }
        let tag = Tag::new(label);
        let id = *tag.id();
        self.tags.insert(id, tag);
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::EventRange;
    use jiff::civil::DateTime;

    fn dt(s: &str) -> DateTime {
        s.parse().unwrap()
    }

    #[test]
    fn merge_combines_distinct_timelines() {
        let mut a = Timeline::new("a");
        a.add_event(Event::new("first", dt("2024-01-01T00:00:00")))
            .unwrap();
        let mut b = Timeline::new("b");
        b.add_event(Event::new("second", dt("2024-02-01T00:00:00")))
            .unwrap();
        b.add_range(Range::new(
            "span",
            EventRange::Start(dt("2024-03-01T00:00:00")),
        ))
        .unwrap();
        b.add_tag(Tag::new("milestone")).unwrap();
        a.merge(b).unwrap();
        assert_eq!(a.events().len(), 2);
        assert_eq!(a.ranges().len(), 1);
        assert_eq!(a.tags().len(), 1);
    }

    #[test]
    fn merge_rejects_duplicate_event_label() {
        let mut a = Timeline::new("a");
        a.add_event(Event::new("dup", dt("2024-01-01T00:00:00")))
            .unwrap();
        let mut b = Timeline::new("b");
        b.add_event(Event::new("dup", dt("2024-02-01T00:00:00")))
            .unwrap();
        assert!(matches!(
            a.merge(b).unwrap_err(),
            TimelineError::EventLabelExists(_),
        ));
    }

    #[test]
    fn merge_rejects_duplicate_range_label() {
        let mut a = Timeline::new("a");
        a.add_range(Range::new(
            "dup",
            EventRange::Start(dt("2024-01-01T00:00:00")),
        ))
        .unwrap();
        let mut b = Timeline::new("b");
        b.add_range(Range::new(
            "dup",
            EventRange::End(dt("2024-02-01T00:00:00")),
        ))
        .unwrap();
        assert!(matches!(
            a.merge(b).unwrap_err(),
            TimelineError::RangeLabelExists(_),
        ));
    }

    #[test]
    fn merge_rejects_duplicate_tag_label() {
        let mut a = Timeline::new("a");
        a.add_tag(Tag::new("dup")).unwrap();
        let mut b = Timeline::new("b");
        b.add_tag(Tag::new("dup")).unwrap();
        assert!(matches!(
            a.merge(b).unwrap_err(),
            TimelineError::TagLabelExists(_),
        ));
    }
}
