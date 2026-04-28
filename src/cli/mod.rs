mod parse_datetime;

use clap::{Parser, Subcommand};
use parse_datetime::{format_datetime, parse_datetime};
use std::path::PathBuf;
use til::error::{TimelineError, TimelineResult};
use til::timeline::Timeline;
use til::unit::{Event, EventRange, Range, Tag};

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
    Inspect,
    /// Render the timeline.
    Show,
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
}

#[derive(Subcommand)]
enum EventCommand {
    /// Add an event.
    Add { label: String, datetime: String },
    /// Remove an event.
    Remove { label: String },
    /// Tag an event.
    Tag { event_label: String, tag: String },
    /// Remove a tag from an event.
    Untag { event_label: String, tag: String },
    /// List all events.
    List,
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
    },
    /// Remove a range.
    Remove { label: String },
    /// List all ranges.
    List,
}

#[derive(Subcommand)]
enum TagCommand {
    /// Add a tag.
    Add { label: String },
    /// Delete a tag and remove it from all events and ranges.
    Delete { label: String },
    /// List all tags.
    List,
}

fn resolve_path(file: &str) -> PathBuf {
    if file.ends_with(".til") {
        PathBuf::from(file)
    } else {
        PathBuf::from(format!("{file}.til"))
    }
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
        let label = path.file_stem().unwrap().to_str().unwrap();
        Ok(Timeline::new(label))
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

fn print_inspect(path: &PathBuf, timeline: &Timeline) {
    println!("{}", path.display());
    println!("  {} events", timeline.events().len());
    println!("  {} ranges", timeline.ranges().len());
    println!("  {} tags", timeline.tags().len());
}

fn print_show(timeline: &Timeline) {
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
    let ranges: Vec<_> = timeline.ranges().values().collect();
    if !ranges.is_empty() {
        println!("Ranges:");
        for r in &ranges {
            let span = match r.value() {
                EventRange::StartEnd(s, e) => format!(
                    "{} — {}",
                    format_datetime(s.datetime()),
                    format_datetime(e.datetime())
                ),
                EventRange::Start(s) => format!("{} — ...", format_datetime(s.datetime())),
                EventRange::End(e) => format!("... — {}", format_datetime(e.datetime())),
            };
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

fn print_events(timeline: &Timeline) {
    let mut events: Vec<_> = timeline.events().values().collect();
    events.sort_by_key(|e| e.datetime());
    for e in &events {
        println!(
            "{}  {}{}",
            format_datetime(e.datetime()),
            e.label(),
            format_tags(timeline, e.tags())
        );
    }
}

fn print_ranges(timeline: &Timeline) {
    for r in timeline.ranges().values() {
        let span = match r.value() {
            EventRange::StartEnd(s, e) => format!(
                "{} — {}",
                format_datetime(s.datetime()),
                format_datetime(e.datetime())
            ),
            EventRange::Start(s) => format!("{} — ...", format_datetime(s.datetime())),
            EventRange::End(e) => format!("... — {}", format_datetime(e.datetime())),
        };
        println!("{}  {}{}", span, r.label(), format_tags(timeline, r.tags()));
    }
}

fn print_tags(timeline: &Timeline) {
    for t in timeline.tags().values() {
        println!("{}", t.label());
    }
}

impl Cli {
    pub fn run(self) -> TimelineResult<()> {
        let path = resolve_path(&self.file);
        match self.command {
            None => {
                if path.exists() {
                    let timeline = load(&path)?;
                    print_inspect(&path, &timeline);
                } else {
                    let timeline = Timeline::new(path.file_stem().unwrap().to_str().unwrap());
                    save(&path, &timeline)?;
                    println!("Created {}", path.display());
                }
            }
            Some(Command::Inspect) => {
                let timeline = load(&path)?;
                print_inspect(&path, &timeline);
            }
            Some(Command::Show) => {
                let timeline = load(&path)?;
                print_show(&timeline);
            }
            Some(Command::Event { command }) => {
                let mut timeline = load_or_create(&path)?;
                match command {
                    EventCommand::Add { label, datetime } => {
                        let dt = parse_datetime(&datetime)?;
                        let event = Event::new(&label, dt);
                        timeline.add_event(event);
                        save(&path, &timeline)?;
                    }
                    EventCommand::Remove { label } => {
                        timeline.remove_event(&label)?;
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
                    EventCommand::List => {
                        print_events(&timeline);
                    }
                }
            }
            Some(Command::Range { command }) => {
                let mut timeline = load_or_create(&path)?;
                match command {
                    RangeCommand::Add { label, start, end } => {
                        let value = match (start, end) {
                            (Some(s), Some(e)) => {
                                let s_dt = parse_datetime(&s)?;
                                let e_dt = parse_datetime(&e)?;
                                EventRange::StartEnd(
                                    Event::new(&label, s_dt),
                                    Event::new(&label, e_dt),
                                )
                            }
                            (Some(s), None) => {
                                let s_dt = parse_datetime(&s)?;
                                EventRange::Start(Event::new(&label, s_dt))
                            }
                            (None, Some(e)) => {
                                let e_dt = parse_datetime(&e)?;
                                EventRange::End(Event::new(&label, e_dt))
                            }
                            (None, None) => return Err(TimelineError::RangeMissingBound),
                        };
                        let range = Range::new(&label, value);
                        timeline.add_range(range);
                        save(&path, &timeline)?;
                    }
                    RangeCommand::Remove { label } => {
                        timeline.remove_range(&label);
                        save(&path, &timeline)?;
                    }
                    RangeCommand::List => {
                        print_ranges(&timeline);
                    }
                }
            }
            Some(Command::Tag { command }) => {
                let mut timeline = load_or_create(&path)?;
                match command {
                    TagCommand::Add { label } => {
                        let tag = Tag::new(&label);
                        timeline.add_tag(tag);
                        save(&path, &timeline)?;
                    }
                    TagCommand::Delete { label } => {
                        timeline.delete_tag(&label)?;
                        save(&path, &timeline)?;
                    }
                    TagCommand::List => {
                        print_tags(&timeline);
                    }
                }
            }
            Some(Command::Merge { files }) => {
                let mut timeline = load_or_create(&path)?;
                for f in &files {
                    let other_path = resolve_path(f);
                    let other = load(&other_path)?;
                    timeline.merge(other);
                }
                save(&path, &timeline)?;
                println!("Merged {} file(s) into {}", files.len(), path.display());
            }
        }
        Ok(())
    }
}
