use derive_getters::Getters;
use jiff::civil::DateTime;
use jiff::tz::TimeZone;
use std::collections::HashSet;

pub type Hash = [u8; 32];

pub type TagId = Hash;

#[derive(Debug, Clone, Eq, PartialEq, Getters)]
pub struct Event {
    datetime: DateTime,
    time_zone: Option<TimeZone>,
    tags: HashSet<TagId>,
    label: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Tag {
    label: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Range {
    value: EventRange,
    tags: HashSet<TagId>,
    label: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EventRange {
    StartEnd((Event, Event)),
    Start(Event),
    End(Event),
}

impl Tag {
    pub fn new(label: &str) -> Self {
        todo!()
    }
}

impl Event {
    pub fn add_tag(&mut self, tag: Tag) {
        todo!()
    }

    pub fn remove_tag(&mut self, tag: Tag) {
        todo!()
    }
}
