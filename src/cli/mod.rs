use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "til", about = "Create and manage timelines")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new timeline file.
    New { file: String },
    /// Manage events.
    Event {
        /// Path to the .til file.
        file: String,
        #[command(subcommand)]
        command: EventCommand,
    },
    /// Manage ranges.
    Range {
        /// Path to the .til file.
        file: String,
        #[command(subcommand)]
        command: RangeCommand,
    },
    /// Manage tags.
    Tag {
        /// Path to the .til file.
        file: String,
        #[command(subcommand)]
        command: TagCommand,
    },
    /// Show the timeline.
    Show {
        /// Path to the .til file.
        file: String,
    },
    /// Merge multiple timeline files into one.
    Merge {
        /// Path to the output .til file.
        output: String,
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
            Command::New { file } => todo!(),
            Command::Event { file, command } => match command {
                EventCommand::Add { label, datetime } => todo!(),
                EventCommand::Remove { label } => todo!(),
                EventCommand::Tag { event_label, tag } => todo!(),
                EventCommand::Untag { event_label, tag } => todo!(),
                EventCommand::List => todo!(),
            },
            Command::Range { file, command } => match command {
                RangeCommand::Add { label, start, end } => todo!(),
                RangeCommand::Remove { label } => todo!(),
                RangeCommand::List => todo!(),
            },
            Command::Tag { file, command } => match command {
                TagCommand::Add { label } => todo!(),
                TagCommand::Delete { label } => todo!(),
                TagCommand::List => todo!(),
            },
            Command::Show { file } => todo!(),
            Command::Merge { output, files } => todo!(),
        }
    }
}
