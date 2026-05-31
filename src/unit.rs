use derive_getters::Getters;
use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Getters, Serialize, Deserialize)]
pub struct Event {
    id: Uuid,
    datetime: DateTime,
    tags: HashSet<Uuid>,
    label: String,
    r#ref: Option<String>,
    /// Opaque attributes blob; conventionally JSON text.
    attributes: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Getters, Serialize, Deserialize)]
pub struct Tag {
    id: Uuid,
    label: String,
}

#[derive(Debug, Clone, PartialEq, Getters, Serialize, Deserialize)]
pub struct Range {
    id: Uuid,
    value: EventRange,
    tags: HashSet<Uuid>,
    label: String,
    r#ref: Option<String>,
    /// Opaque attributes blob; conventionally JSON text.
    attributes: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum EventRange {
    StartEnd(DateTime, DateTime),
    Start(DateTime),
    End(DateTime),
}

impl Tag {
    pub fn new(label: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            label: label.to_string(),
        }
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }
}

impl Event {
    pub fn new(label: &str, datetime: DateTime) -> Self {
        Self {
            id: Uuid::now_v7(),
            datetime,
            tags: HashSet::new(),
            label: label.to_string(),
            r#ref: None,
            attributes: None,
        }
    }

    pub fn add_tag(&mut self, tag_id: Uuid) {
        self.tags.insert(tag_id);
    }

    pub fn remove_tag(&mut self, tag_id: &Uuid) {
        self.tags.remove(tag_id);
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn set_datetime(&mut self, datetime: DateTime) {
        self.datetime = datetime;
    }

    pub fn set_ref(&mut self, value: Option<String>) {
        self.r#ref = value;
    }

    pub fn set_attributes(&mut self, value: Option<String>) {
        self.attributes = value;
    }
}

impl Range {
    pub fn new(label: &str, value: EventRange) -> Self {
        Self {
            id: Uuid::now_v7(),
            value,
            tags: HashSet::new(),
            label: label.to_string(),
            r#ref: None,
            attributes: None,
        }
    }

    pub fn add_tag(&mut self, tag_id: Uuid) {
        self.tags.insert(tag_id);
    }

    pub fn remove_tag(&mut self, tag_id: &Uuid) {
        self.tags.remove(tag_id);
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn set_value(&mut self, value: EventRange) {
        self.value = value;
    }

    pub fn set_ref(&mut self, value: Option<String>) {
        self.r#ref = value;
    }

    pub fn set_attributes(&mut self, value: Option<String>) {
        self.attributes = value;
    }
}
