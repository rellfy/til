use crate::error::{TimelineError, TimelineResult};
use crate::unit::{Event, EventRange, Range, Tag};
use derive_getters::Getters;
use jiff::civil::DateTime;
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

/// How to locate an item, by its user-chosen label or its stable UUID.
#[derive(Debug, Clone, Copy)]
pub enum Selector<'a> {
    Label(&'a str),
    Id(Uuid),
}

/// Patch semantics for an optional field: keep the current value, set a new
/// value, or clear it.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldUpdate<T> {
    Keep,
    Set(T),
    Clear,
}

impl<T> Default for FieldUpdate<T> {
    fn default() -> Self {
        Self::Keep
    }
}

#[derive(Debug, Default)]
pub struct EventChanges {
    pub label: Option<String>,
    pub datetime: Option<DateTime>,
    pub r#ref: FieldUpdate<String>,
    pub attributes: FieldUpdate<String>,
}

#[derive(Debug, Default)]
pub struct RangeChanges {
    pub label: Option<String>,
    pub value: Option<EventRange>,
    pub r#ref: FieldUpdate<String>,
    pub attributes: FieldUpdate<String>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EventFilter<'a> {
    pub tag: Option<&'a str>,
    pub from: Option<DateTime>,
    pub to: Option<DateTime>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RangeFilter<'a> {
    pub tag: Option<&'a str>,
    pub from: Option<DateTime>,
    pub to: Option<DateTime>,
}

fn range_sort_key(value: &EventRange) -> DateTime {
    match value {
        EventRange::StartEnd(s, _) | EventRange::Start(s) => *s,
        EventRange::End(e) => *e,
    }
}

fn does_range_overlap(value: &EventRange, from: Option<DateTime>, to: Option<DateTime>) -> bool {
    let (range_start_opt, range_end_opt) = match value {
        EventRange::StartEnd(s, e) => (Some(*s), Some(*e)),
        EventRange::Start(s) => (Some(*s), None),
        EventRange::End(e) => (None, Some(*e)),
    };
    if let (Some(rs), Some(fe)) = (range_start_opt, to)
        && rs > fe
    {
        return false;
    }
    if let (Some(fs), Some(re)) = (from, range_end_opt)
        && fs > re
    {
        return false;
    }
    true
}

fn selector_label(sel: Selector<'_>) -> String {
    match sel {
        Selector::Label(l) => l.to_string(),
        Selector::Id(id) => id.to_string(),
    }
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

    pub fn find_event(&self, sel: Selector<'_>) -> Option<&Event> {
        match sel {
            Selector::Label(l) => self.events.values().find(|e| e.label() == l),
            Selector::Id(id) => self.events.get(&id),
        }
    }

    pub fn find_range(&self, sel: Selector<'_>) -> Option<&Range> {
        match sel {
            Selector::Label(l) => self.ranges.values().find(|r| r.label() == l),
            Selector::Id(id) => self.ranges.get(&id),
        }
    }

    pub fn find_tag(&self, sel: Selector<'_>) -> Option<&Tag> {
        match sel {
            Selector::Label(l) => self.tags.values().find(|t| t.label() == l),
            Selector::Id(id) => self.tags.get(&id),
        }
    }

    pub fn update_event(&mut self, sel: Selector<'_>, changes: EventChanges) -> TimelineResult<()> {
        let target_id = self
            .find_event(sel)
            .map(|e| *e.id())
            .ok_or_else(|| TimelineError::EventNotFound(selector_label(sel)))?;
        if let Some(new_label) = &changes.label
            && self
                .events
                .values()
                .any(|e| e.id() != &target_id && e.label() == new_label)
        {
            return Err(TimelineError::EventLabelExists(new_label.clone()));
        }
        let event = self.events.get_mut(&target_id).expect("just found");
        if let Some(l) = changes.label {
            event.set_label(l);
        }
        if let Some(d) = changes.datetime {
            event.set_datetime(d);
        }
        match changes.r#ref {
            FieldUpdate::Keep => {}
            FieldUpdate::Set(v) => event.set_ref(Some(v)),
            FieldUpdate::Clear => event.set_ref(None),
        }
        match changes.attributes {
            FieldUpdate::Keep => {}
            FieldUpdate::Set(v) => event.set_attributes(Some(v)),
            FieldUpdate::Clear => event.set_attributes(None),
        }
        Ok(())
    }

    pub fn update_range(&mut self, sel: Selector<'_>, changes: RangeChanges) -> TimelineResult<()> {
        let target_id = self
            .find_range(sel)
            .map(|r| *r.id())
            .ok_or_else(|| TimelineError::RangeNotFound(selector_label(sel)))?;
        if let Some(new_label) = &changes.label
            && self
                .ranges
                .values()
                .any(|r| r.id() != &target_id && r.label() == new_label)
        {
            return Err(TimelineError::RangeLabelExists(new_label.clone()));
        }
        let range = self.ranges.get_mut(&target_id).expect("just found");
        if let Some(l) = changes.label {
            range.set_label(l);
        }
        if let Some(v) = changes.value {
            range.set_value(v);
        }
        match changes.r#ref {
            FieldUpdate::Keep => {}
            FieldUpdate::Set(v) => range.set_ref(Some(v)),
            FieldUpdate::Clear => range.set_ref(None),
        }
        match changes.attributes {
            FieldUpdate::Keep => {}
            FieldUpdate::Set(v) => range.set_attributes(Some(v)),
            FieldUpdate::Clear => range.set_attributes(None),
        }
        Ok(())
    }

    pub fn rename_tag(&mut self, sel: Selector<'_>, new_label: String) -> TimelineResult<()> {
        let target_id = self
            .find_tag(sel)
            .map(|t| *t.id())
            .ok_or_else(|| TimelineError::TagNotFound(selector_label(sel)))?;
        if self
            .tags
            .values()
            .any(|t| t.id() != &target_id && t.label() == &new_label)
        {
            return Err(TimelineError::TagLabelExists(new_label));
        }
        let tag = self.tags.get_mut(&target_id).expect("just found");
        tag.set_label(new_label);
        Ok(())
    }

    pub fn query_events(&self, filter: EventFilter<'_>) -> Vec<&Event> {
        // Outer Option: was a tag arg passed? Inner Option: did it resolve?
        let filter_tag_arg_opt = filter.tag.map(|t| {
            self.tags
                .values()
                .find(|tg| tg.label() == t)
                .map(|tg| *tg.id())
        });
        if let Some(None) = filter_tag_arg_opt {
            return Vec::new();
        }
        // Flattened: Some(id) means "only this tag", None means "no tag filter".
        let filter_tag_opt = filter_tag_arg_opt.flatten();
        let mut events: Vec<&Event> = self
            .events
            .values()
            .filter(|e| {
                if let Some(tid) = filter_tag_opt
                    && !e.tags().contains(&tid)
                {
                    return false;
                }
                if let Some(from) = filter.from
                    && *e.datetime() < from
                {
                    return false;
                }
                if let Some(to) = filter.to
                    && *e.datetime() > to
                {
                    return false;
                }
                true
            })
            .collect();
        events.sort_by_key(|e| *e.datetime());
        events
    }

    pub fn query_ranges(&self, filter: RangeFilter<'_>) -> Vec<&Range> {
        // Outer Option: was a tag arg passed? Inner Option: did it resolve?
        let filter_tag_arg_opt = filter.tag.map(|t| {
            self.tags
                .values()
                .find(|tg| tg.label() == t)
                .map(|tg| *tg.id())
        });
        if let Some(None) = filter_tag_arg_opt {
            return Vec::new();
        }
        // Flattened: Some(id) means "only this tag", None means "no tag filter".
        let filter_tag_opt = filter_tag_arg_opt.flatten();
        let mut ranges: Vec<&Range> = self
            .ranges
            .values()
            .filter(|r| {
                if let Some(tid) = filter_tag_opt
                    && !r.tags().contains(&tid)
                {
                    return false;
                }
                does_range_overlap(r.value(), filter.from, filter.to)
            })
            .collect();
        ranges.sort_by_key(|r| range_sort_key(r.value()));
        ranges
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

    fn populated_timeline() -> Timeline {
        let mut t = Timeline::new("test");
        t.add_event(Event::new("alpha", dt("2020-01-15T00:00:00")))
            .unwrap();
        t.add_event(Event::new("bravo", dt("2022-06-15T00:00:00")))
            .unwrap();
        t.add_event(Event::new("charlie", dt("2024-03-10T00:00:00")))
            .unwrap();
        t.add_range(Range::new(
            "early",
            EventRange::StartEnd(dt("2019-01-01T00:00:00"), dt("2021-12-31T00:00:00")),
        ))
        .unwrap();
        t.add_range(Range::new(
            "ongoing",
            EventRange::Start(dt("2023-01-01T00:00:00")),
        ))
        .unwrap();
        t.add_range(Range::new(
            "ancient",
            EventRange::End(dt("2010-01-01T00:00:00")),
        ))
        .unwrap();
        t.add_tag(Tag::new("milestone")).unwrap();
        t.add_tag(Tag::new("draft")).unwrap();
        t.tag_event("milestone", "bravo").unwrap();
        t.tag_range("milestone", "early").unwrap();
        t
    }

    #[test]
    fn find_event_by_label() {
        let t = populated_timeline();
        let found = t.find_event(Selector::Label("alpha")).unwrap();
        assert_eq!(found.label(), "alpha");
    }

    #[test]
    fn find_event_by_id() {
        let t = populated_timeline();
        let alpha_id = *t.find_event(Selector::Label("alpha")).unwrap().id();
        let by_id = t.find_event(Selector::Id(alpha_id)).unwrap();
        assert_eq!(by_id.label(), "alpha");
    }

    #[test]
    fn find_event_returns_none() {
        let t = populated_timeline();
        assert!(t.find_event(Selector::Label("zzz")).is_none());
        assert!(t.find_event(Selector::Id(Uuid::nil())).is_none());
    }

    #[test]
    fn find_range_by_label_and_id() {
        let t = populated_timeline();
        let r = t.find_range(Selector::Label("early")).unwrap();
        let by_id = t.find_range(Selector::Id(*r.id())).unwrap();
        assert_eq!(by_id.label(), "early");
        assert!(t.find_range(Selector::Label("missing")).is_none());
    }

    #[test]
    fn find_tag_by_label_and_id() {
        let t = populated_timeline();
        let tag = t.find_tag(Selector::Label("milestone")).unwrap();
        let by_id = t.find_tag(Selector::Id(*tag.id())).unwrap();
        assert_eq!(by_id.label(), "milestone");
        assert!(t.find_tag(Selector::Label("missing")).is_none());
    }

    #[test]
    fn update_event_label() {
        let mut t = populated_timeline();
        t.update_event(
            Selector::Label("alpha"),
            EventChanges {
                label: Some("alpha-renamed".to_string()),
                ..Default::default()
            },
        )
        .unwrap();
        assert!(t.find_event(Selector::Label("alpha-renamed")).is_some());
        assert!(t.find_event(Selector::Label("alpha")).is_none());
    }

    #[test]
    fn update_event_preserves_id() {
        let mut t = populated_timeline();
        let original_id = *t.find_event(Selector::Label("alpha")).unwrap().id();
        t.update_event(
            Selector::Label("alpha"),
            EventChanges {
                label: Some("alpha2".to_string()),
                datetime: Some(dt("2021-01-01T00:00:00")),
                ..Default::default()
            },
        )
        .unwrap();
        let after = t.find_event(Selector::Label("alpha2")).unwrap();
        assert_eq!(after.id(), &original_id);
    }

    #[test]
    fn update_event_datetime() {
        let mut t = populated_timeline();
        t.update_event(
            Selector::Label("alpha"),
            EventChanges {
                datetime: Some(dt("2030-12-31T00:00:00")),
                ..Default::default()
            },
        )
        .unwrap();
        let e = t.find_event(Selector::Label("alpha")).unwrap();
        assert_eq!(*e.datetime(), dt("2030-12-31T00:00:00"));
    }

    #[test]
    fn update_event_set_and_clear_ref() {
        let mut t = populated_timeline();
        t.update_event(
            Selector::Label("alpha"),
            EventChanges {
                r#ref: FieldUpdate::Set("https://x".to_string()),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(
            t.find_event(Selector::Label("alpha"))
                .unwrap()
                .r#ref()
                .as_deref(),
            Some("https://x"),
        );
        t.update_event(
            Selector::Label("alpha"),
            EventChanges {
                r#ref: FieldUpdate::Clear,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(
            t.find_event(Selector::Label("alpha"))
                .unwrap()
                .r#ref()
                .is_none()
        );
    }

    #[test]
    fn update_event_set_and_clear_attributes() {
        let mut t = populated_timeline();
        t.update_event(
            Selector::Label("alpha"),
            EventChanges {
                attributes: FieldUpdate::Set("{\"a\":1}".to_string()),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(
            t.find_event(Selector::Label("alpha"))
                .unwrap()
                .attributes()
                .as_deref(),
            Some("{\"a\":1}"),
        );
        t.update_event(
            Selector::Label("alpha"),
            EventChanges {
                attributes: FieldUpdate::Clear,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(
            t.find_event(Selector::Label("alpha"))
                .unwrap()
                .attributes()
                .is_none()
        );
    }

    #[test]
    fn update_event_keep_preserves() {
        let mut t = populated_timeline();
        t.update_event(
            Selector::Label("alpha"),
            EventChanges {
                r#ref: FieldUpdate::Set("https://keep".to_string()),
                ..Default::default()
            },
        )
        .unwrap();
        t.update_event(
            Selector::Label("alpha"),
            EventChanges {
                label: Some("alpha-new".to_string()),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(
            t.find_event(Selector::Label("alpha-new"))
                .unwrap()
                .r#ref()
                .as_deref(),
            Some("https://keep"),
        );
    }

    #[test]
    fn update_event_by_id() {
        let mut t = populated_timeline();
        let id = *t.find_event(Selector::Label("alpha")).unwrap().id();
        t.update_event(
            Selector::Id(id),
            EventChanges {
                label: Some("alpha-via-id".to_string()),
                ..Default::default()
            },
        )
        .unwrap();
        assert!(t.find_event(Selector::Label("alpha-via-id")).is_some());
    }

    #[test]
    fn update_event_rejects_label_collision() {
        let mut t = populated_timeline();
        let err = t
            .update_event(
                Selector::Label("alpha"),
                EventChanges {
                    label: Some("bravo".to_string()),
                    ..Default::default()
                },
            )
            .unwrap_err();
        assert!(matches!(err, TimelineError::EventLabelExists(_)));
    }

    #[test]
    fn update_event_allows_no_op_self_label() {
        let mut t = populated_timeline();
        t.update_event(
            Selector::Label("alpha"),
            EventChanges {
                label: Some("alpha".to_string()),
                ..Default::default()
            },
        )
        .unwrap();
        assert!(t.find_event(Selector::Label("alpha")).is_some());
    }

    #[test]
    fn update_event_not_found() {
        let mut t = populated_timeline();
        let err = t
            .update_event(Selector::Label("zzz"), EventChanges::default())
            .unwrap_err();
        assert!(matches!(err, TimelineError::EventNotFound(_)));
    }

    #[test]
    fn update_range_label_and_value() {
        let mut t = populated_timeline();
        t.update_range(
            Selector::Label("early"),
            RangeChanges {
                label: Some("early-renamed".to_string()),
                value: Some(EventRange::Start(dt("2018-01-01T00:00:00"))),
                ..Default::default()
            },
        )
        .unwrap();
        let r = t.find_range(Selector::Label("early-renamed")).unwrap();
        assert!(matches!(r.value(), EventRange::Start(_)));
    }

    #[test]
    fn update_range_set_and_clear_ref() {
        let mut t = populated_timeline();
        t.update_range(
            Selector::Label("early"),
            RangeChanges {
                r#ref: FieldUpdate::Set("https://r".to_string()),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(
            t.find_range(Selector::Label("early"))
                .unwrap()
                .r#ref()
                .as_deref(),
            Some("https://r"),
        );
        t.update_range(
            Selector::Label("early"),
            RangeChanges {
                r#ref: FieldUpdate::Clear,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(
            t.find_range(Selector::Label("early"))
                .unwrap()
                .r#ref()
                .is_none()
        );
    }

    #[test]
    fn update_range_by_id() {
        let mut t = populated_timeline();
        let id = *t.find_range(Selector::Label("early")).unwrap().id();
        t.update_range(
            Selector::Id(id),
            RangeChanges {
                label: Some("early-via-id".to_string()),
                ..Default::default()
            },
        )
        .unwrap();
        assert!(t.find_range(Selector::Label("early-via-id")).is_some());
    }

    #[test]
    fn update_range_rejects_collision() {
        let mut t = populated_timeline();
        let err = t
            .update_range(
                Selector::Label("early"),
                RangeChanges {
                    label: Some("ongoing".to_string()),
                    ..Default::default()
                },
            )
            .unwrap_err();
        assert!(matches!(err, TimelineError::RangeLabelExists(_)));
    }

    #[test]
    fn update_range_not_found() {
        let mut t = populated_timeline();
        let err = t
            .update_range(Selector::Label("zzz"), RangeChanges::default())
            .unwrap_err();
        assert!(matches!(err, TimelineError::RangeNotFound(_)));
    }

    #[test]
    fn rename_tag_basic() {
        let mut t = populated_timeline();
        t.rename_tag(Selector::Label("milestone"), "milestone2".to_string())
            .unwrap();
        assert!(t.find_tag(Selector::Label("milestone2")).is_some());
        assert!(t.find_tag(Selector::Label("milestone")).is_none());
    }

    #[test]
    fn rename_tag_preserves_associations() {
        let mut t = populated_timeline();
        let tag_id = *t.find_tag(Selector::Label("milestone")).unwrap().id();
        t.rename_tag(Selector::Label("milestone"), "ms2".to_string())
            .unwrap();
        let bravo = t.find_event(Selector::Label("bravo")).unwrap();
        assert!(bravo.tags().contains(&tag_id));
        let early = t.find_range(Selector::Label("early")).unwrap();
        assert!(early.tags().contains(&tag_id));
    }

    #[test]
    fn rename_tag_by_id() {
        let mut t = populated_timeline();
        let id = *t.find_tag(Selector::Label("milestone")).unwrap().id();
        t.rename_tag(Selector::Id(id), "via-id".to_string())
            .unwrap();
        assert!(t.find_tag(Selector::Label("via-id")).is_some());
    }

    #[test]
    fn rename_tag_rejects_collision() {
        let mut t = populated_timeline();
        let err = t
            .rename_tag(Selector::Label("milestone"), "draft".to_string())
            .unwrap_err();
        assert!(matches!(err, TimelineError::TagLabelExists(_)));
    }

    #[test]
    fn rename_tag_not_found() {
        let mut t = populated_timeline();
        let err = t
            .rename_tag(Selector::Label("zzz"), "x".to_string())
            .unwrap_err();
        assert!(matches!(err, TimelineError::TagNotFound(_)));
    }

    #[test]
    fn query_events_no_filter_returns_all_sorted() {
        let t = populated_timeline();
        let events = t.query_events(EventFilter::default());
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].label(), "alpha");
        assert_eq!(events[1].label(), "bravo");
        assert_eq!(events[2].label(), "charlie");
    }

    #[test]
    fn query_events_filter_by_tag() {
        let t = populated_timeline();
        let events = t.query_events(EventFilter {
            tag: Some("milestone"),
            ..Default::default()
        });
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].label(), "bravo");
    }

    #[test]
    fn query_events_filter_by_from() {
        let t = populated_timeline();
        let events = t.query_events(EventFilter {
            from: Some(dt("2022-01-01T00:00:00")),
            ..Default::default()
        });
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].label(), "bravo");
        assert_eq!(events[1].label(), "charlie");
    }

    #[test]
    fn query_events_filter_by_to() {
        let t = populated_timeline();
        let events = t.query_events(EventFilter {
            to: Some(dt("2023-01-01T00:00:00")),
            ..Default::default()
        });
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn query_events_filter_by_range() {
        let t = populated_timeline();
        let events = t.query_events(EventFilter {
            from: Some(dt("2021-01-01T00:00:00")),
            to: Some(dt("2023-01-01T00:00:00")),
            ..Default::default()
        });
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].label(), "bravo");
    }

    #[test]
    fn query_events_unknown_tag_returns_empty() {
        let t = populated_timeline();
        let events = t.query_events(EventFilter {
            tag: Some("nope"),
            ..Default::default()
        });
        assert!(events.is_empty());
    }

    #[test]
    fn query_events_combines_filters() {
        let t = populated_timeline();
        let events = t.query_events(EventFilter {
            tag: Some("milestone"),
            from: Some(dt("2023-01-01T00:00:00")),
            ..Default::default()
        });
        assert!(events.is_empty());
    }

    #[test]
    fn query_ranges_no_filter_returns_all_sorted() {
        let t = populated_timeline();
        let ranges = t.query_ranges(RangeFilter::default());
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0].label(), "ancient");
        assert_eq!(ranges[1].label(), "early");
        assert_eq!(ranges[2].label(), "ongoing");
    }

    #[test]
    fn query_ranges_overlap_start_end() {
        let t = populated_timeline();
        let ranges = t.query_ranges(RangeFilter {
            from: Some(dt("2020-01-01T00:00:00")),
            to: Some(dt("2020-06-01T00:00:00")),
            ..Default::default()
        });
        let labels: Vec<&str> = ranges.iter().map(|r| r.label().as_str()).collect();
        assert_eq!(labels, vec!["early"]);
    }

    #[test]
    fn query_ranges_overlap_start_only() {
        let t = populated_timeline();
        let ranges = t.query_ranges(RangeFilter {
            from: Some(dt("2025-01-01T00:00:00")),
            to: Some(dt("2025-12-31T00:00:00")),
            ..Default::default()
        });
        let labels: Vec<&str> = ranges.iter().map(|r| r.label().as_str()).collect();
        assert_eq!(labels, vec!["ongoing"]);
    }

    #[test]
    fn query_ranges_overlap_end_only() {
        let t = populated_timeline();
        let ranges = t.query_ranges(RangeFilter {
            from: Some(dt("2005-01-01T00:00:00")),
            to: Some(dt("2008-01-01T00:00:00")),
            ..Default::default()
        });
        let labels: Vec<&str> = ranges.iter().map(|r| r.label().as_str()).collect();
        assert_eq!(labels, vec!["ancient"]);
    }

    #[test]
    fn query_ranges_no_overlap() {
        let t = populated_timeline();
        let ranges = t.query_ranges(RangeFilter {
            from: Some(dt("2011-01-01T00:00:00")),
            to: Some(dt("2011-12-31T00:00:00")),
            ..Default::default()
        });
        assert!(ranges.is_empty());
    }

    #[test]
    fn query_ranges_filter_by_tag() {
        let t = populated_timeline();
        let ranges = t.query_ranges(RangeFilter {
            tag: Some("milestone"),
            ..Default::default()
        });
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].label(), "early");
    }

    #[test]
    fn query_ranges_unknown_tag_returns_empty() {
        let t = populated_timeline();
        let ranges = t.query_ranges(RangeFilter {
            tag: Some("nope"),
            ..Default::default()
        });
        assert!(ranges.is_empty());
    }
}
