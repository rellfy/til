use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "til", about = "Create and manage timelines")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new timeline.
    New { name: String },
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
    /// Show the timeline.
    Show,
}

#[derive(Subcommand)]
enum EventCommand {
    /// Add an event.
    Add { label: String, datetime: String },
    /// Remove an event.
    Remove { label: String },
    /// Tag an event.
    Tag { event_label: String, tag: String },
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
}

#[derive(Subcommand)]
enum TagCommand {
    /// List all tags.
    List,
}

impl Cli {
    pub fn run(self) {
        match self.command {
            Command::New { name } => todo!(),
            Command::Event { command } => match command {
                EventCommand::Add { label, datetime } => todo!(),
                EventCommand::Remove { label } => todo!(),
                EventCommand::Tag { event_label, tag } => todo!(),
            },
            Command::Range { command } => match command {
                RangeCommand::Add { label, start, end } => todo!(),
                RangeCommand::Remove { label } => todo!(),
            },
            Command::Tag { command } => match command {
                TagCommand::List => todo!(),
            },
            Command::Show => todo!(),
        }
    }
}
