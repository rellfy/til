use crate::error::{TimelineError, TimelineResult};
use crate::unit::{Event, Range, Tag};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn add_event(&mut self, event: Event) {
        self.events.insert(*event.id(), event);
    }

    pub fn remove_event(&mut self, label: &str) {
        self.events.retain(|_, e| e.label() != label);
    }

    pub fn tag_event(&mut self, tag_label: &str, event_label: &str) -> TimelineResult<()> {
        let tag = Tag::new(tag_label);
        let tag_id = *tag.id();
        self.tags.insert(tag_id, tag);
        let event = self
            .events
            .values_mut()
            .find(|e| e.label() == event_label)
            .ok_or(TimelineError::EventNotFound)?;
        event.add_tag(tag_id);
        Ok(())
    }

    pub fn add_range(&mut self, range: Range) {
        self.ranges.insert(*range.id(), range);
    }

    pub fn remove_range(&mut self, label: &str) {
        self.ranges.retain(|_, r| r.label() != label);
    }

    fn remove_unused_tags(&mut self) {
        todo!()
    }
}
