# til

til is a program for creating and managing timelines.

til can create timelines with events and ranges, all of which can be tagged for
grouping and organising occurrences.

til treats timelines as files, with the `.til` extension.

til can be used for all sorts of things:

- Logging incidents in systems
- Personal events, such as family events and history
- As an aid to study history and keep track of historical events
- As a convenient way to build and share a timeline of anything!

## Local Usage

til is a command line utility. Every invocation takes the path to a `.til` file
as its first argument; the `.til` extension may be omitted.

To create a new timeline, run til with no subcommand:

```
til half-life-3
```

If `half-life-3.til` does not exist, it is created. If it already exists, til
prints a summary (event/range/tag counts).

To add an event:

```
til half-life-3 event add "first person to think about half-life 3" 2004-11-16
```

Dates are accepted in several forms:

- ISO datetime: `2004-11-16T09:30:00`
- ISO date: `2004-11-16` (defaults to midnight)
- Year/month: `2004-11`
- Compact: `20041116`
- Named month: `November 16 2004`, `Nov 2004`
- Bare year: `2004`

To add a range, supply `--start` and/or `--end` datetimes:

```
til half-life-3 range add "half-life 3 time to release" --start 2004-11-16
```

A range needs at least one of `--start` or `--end`.

To render the timeline:

```
til half-life-3 show
```

This prints events (sorted by datetime), ranges, and tags.

Other commands:

- `til <file> inspect`: counts of events, ranges and tags.
- `til <file> event {remove,tag,untag,list}`: manage events.
- `til <file> range {remove,tag,untag,list}`: manage ranges.
- `til <file> tag {add,delete,list}`: manage tags.
- `til <file> merge <other.til> [...]`: merge other timelines into this one.

## .til file

The `.til` file is a binary file that stores data for a single timeline.
