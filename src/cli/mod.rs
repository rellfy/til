use clap::{Parser, Subcommand};

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
        start: String,
        #[arg(long)]
        end: String,
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

impl Cli {
    pub fn run(self) {
        match self.command {
            None => todo!(),
            Some(Command::Inspect) => todo!(),
            Some(Command::Show) => todo!(),
            Some(Command::Event { command }) => match command {
                EventCommand::Add { label, datetime } => todo!(),
                EventCommand::Remove { label } => todo!(),
                EventCommand::Tag { event_label, tag } => todo!(),
                EventCommand::Untag { event_label, tag } => todo!(),
                EventCommand::List => todo!(),
            },
            Some(Command::Range { command }) => match command {
                RangeCommand::Add { label, start, end } => todo!(),
                RangeCommand::Remove { label } => todo!(),
                RangeCommand::List => todo!(),
            },
            Some(Command::Tag { command }) => match command {
                TagCommand::Add { label } => todo!(),
                TagCommand::Delete { label } => todo!(),
                TagCommand::List => todo!(),
            },
            Some(Command::Merge { files }) => todo!(),
        }
    }
}
