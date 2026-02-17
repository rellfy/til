use derive_getters::Getters;
use jiff::civil::DateTime;
use jiff::tz::TimeZone;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Getters, Serialize, Deserialize)]
pub struct Event {
    id: Uuid,
    datetime: DateTime,
    #[serde(skip)]
    time_zone: Option<TimeZone>,
    tags: HashSet<Uuid>,
    label: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Getters, Serialize, Deserialize)]
pub struct Tag {
    id: Uuid,
    label: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Getters, Serialize, Deserialize)]
pub struct Range {
    id: Uuid,
    value: EventRange,
    tags: HashSet<Uuid>,
    label: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum EventRange {
    StartEnd(Event, Event),
    Start(Event),
    End(Event),
}

impl Tag {
    pub fn new(label: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            label: label.to_string(),
        }
    }
}

impl Event {
    pub fn new(label: &str, datetime: DateTime, time_zone: Option<TimeZone>) -> Self {
        Self {
            id: Uuid::now_v7(),
            datetime,
            time_zone,
            tags: HashSet::new(),
            label: label.to_string(),
        }
    }

    pub fn add_tag(&mut self, tag_id: Uuid) {
        self.tags.insert(tag_id);
    }

    pub fn remove_tag(&mut self, tag_id: &Uuid) {
        self.tags.remove(tag_id);
    }
}

impl Range {
    pub fn new(label: &str, value: EventRange) -> Self {
        Self {
            id: Uuid::now_v7(),
            value,
            tags: HashSet::new(),
            label: label.to_string(),
        }
    }
}
