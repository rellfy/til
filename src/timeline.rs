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

    pub fn add_event(&mut self, event: Event) {
        self.events.insert(*event.id(), event);
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

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(*tag.id(), tag);
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

    pub fn add_range(&mut self, range: Range) {
        self.ranges.insert(*range.id(), range);
    }

    pub fn remove_range(&mut self, label: &str) {
        self.ranges.retain(|_, r| r.label() != label);
    }

    pub fn merge(&mut self, other: Timeline) {
        self.tags.extend(other.tags);
        self.events.extend(other.events);
        self.ranges.extend(other.ranges);
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
