use clap::{Parser, Subcommand};
use parse_datetime::{format_datetime, parse_datetime};
use std::io::Read;
use std::path::PathBuf;
use til::error::{TimelineError, TimelineResult};
use til::timeline::{
    EventChanges, EventFilter, FieldUpdate, RangeChanges, RangeFilter, Selector, Timeline,
};
use til::unit::{Event, EventRange, Range, Tag};
use uuid::Uuid;

mod parse_datetime;

#[derive(Parser)]
#[command(name = "til", about = "Create and manage timelines")]
pub struct Cli {
    /// Path to the .til file.
    file: String,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Show event, tag and range counts.
    Inspect {
        /// Emit JSON instead of human-readable text.
        #[arg(long)]
        json: bool,
    },
    /// Render the timeline.
    Show {
        /// Emit JSON instead of human-readable text.
        #[arg(long)]
        json: bool,
    },
    /// Manage events.
    Event {
        #[command(subcommand)]
        command: EventCommand,
    },
    /// Manage ranges.
    Range {
        #[command(subcommand)]
        command: RangeCommand,
    },
    /// Manage tags.
    Tag {
        #[command(subcommand)]
        command: TagCommand,
    },
    /// Merge other timeline files into this one.
    Merge {
        /// Paths to the .til files to merge.
        files: Vec<String>,
    },
    /// Replace the timeline with one read from JSON (stdin by default).
    Import {
        /// Path to a JSON file (omit to read stdin).
        #[arg(long)]
        from: Option<PathBuf>,
        /// Merge into the existing timeline instead of replacing it.
        #[arg(long)]
        merge: bool,
    },
}

#[derive(Subcommand)]
enum EventCommand {
    /// Add an event.
    Add {
        label: String,
        datetime: String,
        /// Optional opaque reference (URI, UUID, S3 key, etc.).
        #[arg(long)]
        r#ref: Option<String>,
        /// Optional attributes as a JSON string.
        #[arg(long)]
        attributes: Option<String>,
    },
    /// Remove an event.
    Remove {
        /// Event label (or use --id).
        label: Option<String>,
        /// Event UUID (alternative to label).
        #[arg(long, conflicts_with = "label")]
        id: Option<Uuid>,
    },
    /// Update an event's fields.
    Update {
        /// Event label (or use --id).
        label: Option<String>,
        /// Event UUID (alternative to label).
        #[arg(long, conflicts_with = "label")]
        id: Option<Uuid>,
        /// New label.
        #[arg(long)]
        set_label: Option<String>,
        /// New datetime.
        #[arg(long)]
        set_datetime: Option<String>,
        /// New ref string.
        #[arg(long)]
        set_ref: Option<String>,
        /// Clear the ref.
        #[arg(long, conflicts_with = "set_ref")]
        clear_ref: bool,
        /// New attributes JSON string.
        #[arg(long)]
        set_attributes: Option<String>,
        /// Clear the attributes.
        #[arg(long, conflicts_with = "set_attributes")]
        clear_attributes: bool,
    },
    /// Tag an event.
    Tag { event_label: String, tag: String },
    /// Remove a tag from an event.
    Untag { event_label: String, tag: String },
    /// List events, optionally filtered.
    List {
        /// Only events with this tag label.
        #[arg(long)]
        tag: Option<String>,
        /// Only events on or after this datetime.
        #[arg(long)]
        from: Option<String>,
        /// Only events on or before this datetime.
        #[arg(long)]
        to: Option<String>,
        /// Emit machine-readable JSON instead of text.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum RangeCommand {
    /// Add a range.
    Add {
        label: String,
        #[arg(long)]
        start: Option<String>,
        #[arg(long)]
        end: Option<String>,
        /// Optional opaque reference (URI, UUID, S3 key, etc.).
        #[arg(long)]
        r#ref: Option<String>,
        /// Optional attributes as a JSON string.
        #[arg(long)]
        attributes: Option<String>,
    },
    /// Remove a range.
    Remove {
        /// Range label (or use --id).
        label: Option<String>,
        /// Range UUID (alternative to label).
        #[arg(long, conflicts_with = "label")]
        id: Option<Uuid>,
    },
    /// Update a range's fields.
    Update {
        /// Range label (or use --id).
        label: Option<String>,
        /// Range UUID (alternative to label).
        #[arg(long, conflicts_with = "label")]
        id: Option<Uuid>,
        /// New label.
        #[arg(long)]
        set_label: Option<String>,
        /// New start datetime.
        #[arg(long)]
        set_start: Option<String>,
        /// New end datetime.
        #[arg(long)]
        set_end: Option<String>,
        /// Drop the start (must keep end).
        #[arg(long, conflicts_with = "set_start")]
        clear_start: bool,
        /// Drop the end (must keep start).
        #[arg(long, conflicts_with = "set_end")]
        clear_end: bool,
        /// New ref string.
        #[arg(long)]
        set_ref: Option<String>,
        /// Clear the ref.
        #[arg(long, conflicts_with = "set_ref")]
        clear_ref: bool,
        /// New attributes JSON string.
        #[arg(long)]
        set_attributes: Option<String>,
        /// Clear the attributes.
        #[arg(long, conflicts_with = "set_attributes")]
        clear_attributes: bool,
    },
    /// Tag a range.
    Tag { range_label: String, tag: String },
    /// Remove a tag from a range.
    Untag { range_label: String, tag: String },
    /// List ranges, optionally filtered.
    List {
        /// Only ranges with this tag label.
        #[arg(long)]
        tag: Option<String>,
        /// Only ranges overlapping on or after this datetime.
        #[arg(long)]
        from: Option<String>,
        /// Only ranges overlapping on or before this datetime.
        #[arg(long)]
        to: Option<String>,
        /// Emit machine-readable JSON instead of text.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum TagCommand {
    /// Add a tag.
    Add { label: String },
    /// Delete a tag and remove it from all events and ranges.
    Delete {
        /// Tag label (or use --id).
        label: Option<String>,
        /// Tag UUID (alternative to label).
        #[arg(long, conflicts_with = "label")]
        id: Option<Uuid>,
    },
    /// Rename a tag.
    Rename {
        /// Tag label (or use --id).
        label: Option<String>,
        /// Tag UUID (alternative to label).
        #[arg(long, conflicts_with = "label")]
        id: Option<Uuid>,
        /// New label.
        new_label: String,
    },
    /// List tags.
    List {
        /// Emit machine-readable JSON instead of text.
        #[arg(long)]
        json: bool,
    },
}

fn resolve_path(file: &str) -> PathBuf {
    if file.ends_with(".til") {
        PathBuf::from(file)
    } else {
        PathBuf::from(format!("{file}.til"))
    }
}

fn stem_label(path: &PathBuf) -> TimelineResult<&str> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| TimelineError::InvalidPath(path.display().to_string()))
}

fn load(path: &PathBuf) -> TimelineResult<Timeline> {
    let bytes = std::fs::read(path)?;
    Timeline::from_bytes(&bytes)
}

fn save(path: &PathBuf, timeline: &Timeline) -> TimelineResult<()> {
    std::fs::write(path, timeline.as_bytes()?)?;
    Ok(())
}

fn load_or_create(path: &PathBuf) -> TimelineResult<Timeline> {
    if path.exists() {
        load(path)
    } else {
        Ok(Timeline::new(stem_label(path)?))
    }
}

fn resolve_selector<'a>(label: Option<&'a str>, id: Option<Uuid>) -> TimelineResult<Selector<'a>> {
    match (label, id) {
        (Some(l), None) => Ok(Selector::Label(l)),
        (None, Some(id)) => Ok(Selector::Id(id)),
        (Some(_), Some(_)) => Err(TimelineError::SelectorAmbiguous),
        (None, None) => Err(TimelineError::SelectorMissing),
    }
}

fn validate_attributes_json(s: &str) -> TimelineResult<()> {
    serde_json::from_str::<serde_json::Value>(s)
        .map_err(|e| TimelineError::AttributesParse(e.to_string()))?;
    Ok(())
}

fn range_sort_key(value: &EventRange) -> jiff::civil::DateTime {
    match value {
        EventRange::StartEnd(s, _) | EventRange::Start(s) => *s,
        EventRange::End(e) => *e,
    }
}

fn format_range_span(value: &EventRange) -> String {
    match value {
        EventRange::StartEnd(s, e) => {
            format!("{} - {}", format_datetime(s), format_datetime(e))
        }
        EventRange::Start(s) => format!("{} - ...", format_datetime(s)),
        EventRange::End(e) => format!("... - {}", format_datetime(e)),
    }
}

fn format_tags(timeline: &Timeline, tag_ids: &std::collections::HashSet<uuid::Uuid>) -> String {
    if tag_ids.is_empty() {
        return String::new();
    }
    let labels: Vec<&str> = tag_ids
        .iter()
        .filter_map(|id| timeline.tag_label(id))
        .collect();
    format!("  [{}]", labels.join(", "))
}

fn print_inspect_text(path: &PathBuf, timeline: &Timeline) {
    println!("{}", path.display());
    println!("  {} events", timeline.events().len());
    println!("  {} ranges", timeline.ranges().len());
    println!("  {} tags", timeline.tags().len());
}

fn print_inspect_json(path: &PathBuf, timeline: &Timeline) -> TimelineResult<()> {
    let v = serde_json::json!({
        "path": path.display().to_string(),
        "events": timeline.events().len(),
        "ranges": timeline.ranges().len(),
        "tags": timeline.tags().len(),
    });
    println!("{}", serde_json::to_string_pretty(&v)?);
    Ok(())
}

fn print_show_text(timeline: &Timeline) {
    println!("{}", timeline.label());
    println!();
    let mut events: Vec<_> = timeline.events().values().collect();
    events.sort_by_key(|e| e.datetime());
    if !events.is_empty() {
        println!("Events:");
        for e in &events {
            println!(
                "  {}  {}{}",
                format_datetime(e.datetime()),
                e.label(),
                format_tags(timeline, e.tags())
            );
        }
        println!();
    }
    let mut ranges: Vec<_> = timeline.ranges().values().collect();
    ranges.sort_by_key(|r| range_sort_key(r.value()));
    if !ranges.is_empty() {
        println!("Ranges:");
        for r in &ranges {
            let span = format_range_span(r.value());
            println!(
                "  {}  {}{}",
                span,
                r.label(),
                format_tags(timeline, r.tags())
            );
        }
        println!();
    }
    let tags: Vec<_> = timeline.tags().values().collect();
    if !tags.is_empty() {
        println!("Tags:");
        let labels: Vec<&str> = tags.iter().map(|t| t.label().as_str()).collect();
        println!("  {}", labels.join(", "));
    }
}

fn print_events_text(timeline: &Timeline, events: &[&Event]) {
    for e in events {
        println!(
            "{}  {}{}",
            format_datetime(e.datetime()),
            e.label(),
            format_tags(timeline, e.tags())
        );
    }
}

fn print_events_json(events: &[&Event]) -> TimelineResult<()> {
    println!("{}", serde_json::to_string_pretty(events)?);
    Ok(())
}

fn print_ranges_text(timeline: &Timeline, ranges: &[&Range]) {
    for r in ranges {
        let span = format_range_span(r.value());
        println!("{}  {}{}", span, r.label(), format_tags(timeline, r.tags()));
    }
}

fn print_ranges_json(ranges: &[&Range]) -> TimelineResult<()> {
    println!("{}", serde_json::to_string_pretty(ranges)?);
    Ok(())
}

fn print_tags_text(timeline: &Timeline) {
    for t in timeline.tags().values() {
        println!("{}", t.label());
    }
}

fn print_tags_json(timeline: &Timeline) -> TimelineResult<()> {
    let tags: Vec<&Tag> = timeline.tags().values().collect();
    println!("{}", serde_json::to_string_pretty(&tags)?);
    Ok(())
}

fn apply_range_value_changes(
    existing: &EventRange,
    set_start: Option<jiff::civil::DateTime>,
    set_end: Option<jiff::civil::DateTime>,
    clear_start: bool,
    clear_end: bool,
) -> TimelineResult<Option<EventRange>> {
    if set_start.is_none() && set_end.is_none() && !clear_start && !clear_end {
        return Ok(None);
    }
    let cur_start = match existing {
        EventRange::StartEnd(s, _) | EventRange::Start(s) => Some(*s),
        EventRange::End(_) => None,
    };
    let cur_end = match existing {
        EventRange::StartEnd(_, e) | EventRange::End(e) => Some(*e),
        EventRange::Start(_) => None,
    };
    let new_start = if clear_start {
        None
    } else {
        set_start.or(cur_start)
    };
    let new_end = if clear_end { None } else { set_end.or(cur_end) };
    let value = match (new_start, new_end) {
        (Some(s), Some(e)) => EventRange::StartEnd(s, e),
        (Some(s), None) => EventRange::Start(s),
        (None, Some(e)) => EventRange::End(e),
        (None, None) => return Err(TimelineError::RangeMissingBound),
    };
    Ok(Some(value))
}

fn read_stdin_to_string() -> TimelineResult<String> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

impl Cli {
    pub fn run(self) -> TimelineResult<()> {
        let path = resolve_path(&self.file);
        match self.command {
            None => {
                if path.exists() {
                    let timeline = load(&path)?;
                    print_inspect_text(&path, &timeline);
                } else {
                    let timeline = Timeline::new(stem_label(&path)?);
                    save(&path, &timeline)?;
                    println!("Created {}", path.display());
                }
            }
            Some(Command::Inspect { json }) => {
                let timeline = load(&path)?;
                if json {
                    print_inspect_json(&path, &timeline)?;
                } else {
                    print_inspect_text(&path, &timeline);
                }
            }
            Some(Command::Show { json }) => {
                let timeline = load(&path)?;
                if json {
                    println!("{}", timeline.to_json()?);
                } else {
                    print_show_text(&timeline);
                }
            }
            Some(Command::Event { command }) => {
                let mut timeline = load_or_create(&path)?;
                match command {
                    EventCommand::Add {
                        label,
                        datetime,
                        r#ref,
                        attributes,
                    } => {
                        let dt = parse_datetime(&datetime)?;
                        let mut event = Event::new(&label, dt);
                        if r#ref.is_some() {
                            event.set_ref(r#ref);
                        }
                        if let Some(json) = attributes {
                            validate_attributes_json(&json)?;
                            event.set_attributes(Some(json));
                        }
                        timeline.add_event(event)?;
                        save(&path, &timeline)?;
                    }
                    EventCommand::Remove { label, id } => {
                        let sel = resolve_selector(label.as_deref(), id)?;
                        let target_label = timeline
                            .find_event(sel)
                            .map(|e| e.label().clone())
                            .ok_or_else(|| {
                                TimelineError::EventNotFound(match sel {
                                    Selector::Label(l) => l.to_string(),
                                    Selector::Id(id) => id.to_string(),
                                })
                            })?;
                        timeline.remove_event(&target_label)?;
                        save(&path, &timeline)?;
                    }
                    EventCommand::Update {
                        label,
                        id,
                        set_label,
                        set_datetime,
                        set_ref,
                        clear_ref,
                        set_attributes,
                        clear_attributes,
                    } => {
                        let sel = resolve_selector(label.as_deref(), id)?;
                        let mut changes = EventChanges::default();
                        if let Some(l) = set_label {
                            changes.label = Some(l);
                        }
                        if let Some(d) = set_datetime {
                            changes.datetime = Some(parse_datetime(&d)?);
                        }
                        if let Some(r) = set_ref {
                            changes.r#ref = FieldUpdate::Set(r);
                        } else if clear_ref {
                            changes.r#ref = FieldUpdate::Clear;
                        }
                        if let Some(a) = set_attributes {
                            validate_attributes_json(&a)?;
                            changes.attributes = FieldUpdate::Set(a);
                        } else if clear_attributes {
                            changes.attributes = FieldUpdate::Clear;
                        }
                        timeline.update_event(sel, changes)?;
                        save(&path, &timeline)?;
                    }
                    EventCommand::Tag { event_label, tag } => {
                        timeline.tag_event(&tag, &event_label)?;
                        save(&path, &timeline)?;
                    }
                    EventCommand::Untag { event_label, tag } => {
                        timeline.untag_event(&tag, &event_label)?;
                        save(&path, &timeline)?;
                    }
                    EventCommand::List {
                        tag,
                        from,
                        to,
                        json,
                    } => {
                        let from_dt = from.as_deref().map(parse_datetime).transpose()?;
                        let to_dt = to.as_deref().map(parse_datetime).transpose()?;
                        let filter = EventFilter {
                            tag: tag.as_deref(),
                            from: from_dt,
                            to: to_dt,
                        };
                        let events = timeline.query_events(filter);
                        if json {
                            print_events_json(&events)?;
                        } else {
                            print_events_text(&timeline, &events);
                        }
                    }
                }
            }
            Some(Command::Range { command }) => {
                let mut timeline = load_or_create(&path)?;
                match command {
                    RangeCommand::Add {
                        label,
                        start,
                        end,
                        r#ref,
                        attributes,
                    } => {
                        let value = match (start, end) {
                            (Some(s), Some(e)) => {
                                EventRange::StartEnd(parse_datetime(&s)?, parse_datetime(&e)?)
                            }
                            (Some(s), None) => EventRange::Start(parse_datetime(&s)?),
                            (None, Some(e)) => EventRange::End(parse_datetime(&e)?),
                            (None, None) => return Err(TimelineError::RangeMissingBound),
                        };
                        let mut range = Range::new(&label, value);
                        if r#ref.is_some() {
                            range.set_ref(r#ref);
                        }
                        if let Some(json) = attributes {
                            validate_attributes_json(&json)?;
                            range.set_attributes(Some(json));
                        }
                        timeline.add_range(range)?;
                        save(&path, &timeline)?;
                    }
                    RangeCommand::Remove { label, id } => {
                        let sel = resolve_selector(label.as_deref(), id)?;
                        let target_label = timeline
                            .find_range(sel)
                            .map(|r| r.label().clone())
                            .ok_or_else(|| {
                                TimelineError::RangeNotFound(match sel {
                                    Selector::Label(l) => l.to_string(),
                                    Selector::Id(id) => id.to_string(),
                                })
                            })?;
                        timeline.remove_range(&target_label);
                        save(&path, &timeline)?;
                    }
                    RangeCommand::Update {
                        label,
                        id,
                        set_label,
                        set_start,
                        set_end,
                        clear_start,
                        clear_end,
                        set_ref,
                        clear_ref,
                        set_attributes,
                        clear_attributes,
                    } => {
                        let sel = resolve_selector(label.as_deref(), id)?;
                        let existing_value = timeline
                            .find_range(sel)
                            .map(|r| r.value().clone())
                            .ok_or_else(|| {
                                TimelineError::RangeNotFound(match sel {
                                    Selector::Label(l) => l.to_string(),
                                    Selector::Id(id) => id.to_string(),
                                })
                            })?;
                        let mut changes = RangeChanges::default();
                        if let Some(l) = set_label {
                            changes.label = Some(l);
                        }
                        let new_start_dt = set_start.as_deref().map(parse_datetime).transpose()?;
                        let new_end_dt = set_end.as_deref().map(parse_datetime).transpose()?;
                        changes.value = apply_range_value_changes(
                            &existing_value,
                            new_start_dt,
                            new_end_dt,
                            clear_start,
                            clear_end,
                        )?;
                        if let Some(r) = set_ref {
                            changes.r#ref = FieldUpdate::Set(r);
                        } else if clear_ref {
                            changes.r#ref = FieldUpdate::Clear;
                        }
                        if let Some(a) = set_attributes {
                            validate_attributes_json(&a)?;
                            changes.attributes = FieldUpdate::Set(a);
                        } else if clear_attributes {
                            changes.attributes = FieldUpdate::Clear;
                        }
                        timeline.update_range(sel, changes)?;
                        save(&path, &timeline)?;
                    }
                    RangeCommand::Tag { range_label, tag } => {
                        timeline.tag_range(&tag, &range_label)?;
                        save(&path, &timeline)?;
                    }
                    RangeCommand::Untag { range_label, tag } => {
                        timeline.untag_range(&tag, &range_label)?;
                        save(&path, &timeline)?;
                    }
                    RangeCommand::List {
                        tag,
                        from,
                        to,
                        json,
                    } => {
                        let from_dt = from.as_deref().map(parse_datetime).transpose()?;
                        let to_dt = to.as_deref().map(parse_datetime).transpose()?;
                        let filter = RangeFilter {
                            tag: tag.as_deref(),
                            from: from_dt,
                            to: to_dt,
                        };
                        let ranges = timeline.query_ranges(filter);
                        if json {
                            print_ranges_json(&ranges)?;
                        } else {
                            print_ranges_text(&timeline, &ranges);
                        }
                    }
                }
            }
            Some(Command::Tag { command }) => {
                let mut timeline = load_or_create(&path)?;
                match command {
                    TagCommand::Add { label } => {
                        let tag = Tag::new(&label);
                        timeline.add_tag(tag)?;
                        save(&path, &timeline)?;
                    }
                    TagCommand::Delete { label, id } => {
                        let sel = resolve_selector(label.as_deref(), id)?;
                        let target_label = timeline
                            .find_tag(sel)
                            .map(|t| t.label().clone())
                            .ok_or_else(|| {
                                TimelineError::TagNotFound(match sel {
                                    Selector::Label(l) => l.to_string(),
                                    Selector::Id(id) => id.to_string(),
                                })
                            })?;
                        timeline.delete_tag(&target_label)?;
                        save(&path, &timeline)?;
                    }
                    TagCommand::Rename {
                        label,
                        id,
                        new_label,
                    } => {
                        let sel = resolve_selector(label.as_deref(), id)?;
                        timeline.rename_tag(sel, new_label)?;
                        save(&path, &timeline)?;
                    }
                    TagCommand::List { json } => {
                        if json {
                            print_tags_json(&timeline)?;
                        } else {
                            print_tags_text(&timeline);
                        }
                    }
                }
            }
            Some(Command::Merge { files }) => {
                let mut timeline = load_or_create(&path)?;
                for f in &files {
                    let other_path = resolve_path(f);
                    let other = load(&other_path)?;
                    timeline.merge(other)?;
                }
                save(&path, &timeline)?;
                println!("Merged {} file(s) into {}", files.len(), path.display());
            }
            Some(Command::Import { from, merge }) => {
                let raw = match from {
                    Some(p) => std::fs::read_to_string(&p)?,
                    None => read_stdin_to_string()?,
                };
                let incoming = Timeline::from_json(&raw)?;
                if merge {
                    let mut timeline = load_or_create(&path)?;
                    timeline.merge(incoming)?;
                    save(&path, &timeline)?;
                } else {
                    save(&path, &incoming)?;
                }
                println!("Imported into {}", path.display());
            }
        }
        Ok(())
    }
}
