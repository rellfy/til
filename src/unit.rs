use std::collections::HashSet;
use jiff::civil::DateTime;
use jiff::tz::TimeZone;

pub type Hash = [u8; 32];

pub type TagId = Hash;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Event {
    hash: Hash,
    datetime: DateTime,
    time_zone: Option<TimeZone>,
    tags: HashSet<TagId>,
    label: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Tag {
    hash: Hash,
    label: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Range {
    hash: Hash,
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
