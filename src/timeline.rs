use std::collections::HashSet;
use crate::unit::{Event, Hash, Range, Tag};

pub struct Timeline {
    hash: Hash,
    events: HashSet<Event>,
    ranges: HashSet<Range>,
    tags: HashSet<Tag>,
    label: String,
}
