use crate::error::{TimelineError, TimelineResult};
use crate::unit::{Event, Hash, Range, Tag};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Timeline {
    hash: Hash,
    events: HashSet<Event>,
    ranges: HashSet<Range>,
    tags: HashSet<Tag>,
    label: String,
}

impl Timeline {
    pub fn add_event(&mut self, event: Event) {
        self.events.insert(event);
        self.update_hash();
    }

    pub fn remove_event(&mut self, label: &str) {
        self.events.retain(|e| e.label() != label);
        self.update_hash();
    }

    pub fn tag_event(&mut self, tag: &str, event_label: &str) -> TimelineResult<()> {
        let tag = Tag::new(tag);
        self.tags.insert(tag.clone());
        let event_opt = self.events.iter().find(|e| e.label() == event_label);
        let Some(event) = event_opt else {
            return TimelineError::EventNotFound.into();
        };
        self.events.remove(event);
        let mut new_event = event.clone();
        new_event.add_tag(tag);
        self.events.insert(new_event);
        Ok(())
    }

    pub fn add_range(&mut self, range: Range) {
        self.ranges.insert(range);
        self.update_hash();
    }

    pub fn remove_range(&mut self, label: &str) {
        self.ranges.retain(|e| e.label() != label);
        self.update_hash();
    }

    fn update_hash(&mut self) {
        todo!()
    }

    fn remove_unused_tags(&mut self) {
        todo!()
    }
}
