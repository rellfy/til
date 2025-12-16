use crate::unit::{Event, Hash, Range, Tag};
use std::collections::HashSet;

pub struct Timeline {
    hash: Hash,
    events: HashSet<Event>,
    ranges: HashSet<Range>,
    tags: HashSet<Tag>,
    label: String,
}
