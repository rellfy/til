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

til can be used as a command line utility.

To create a new timeline:

```
til init half-life-3
```

This creates the file `half-life-3.til` and opens it.

To add an event to the newly created timeline:

```
til "first person to think about half-life 3" 2004/11/16
```

This will add the labelled event with a date but without a time zone.
Because no time was provided, this event will default to midnight.

It is also possible to create ranges.
Ranges can link two events, but they can also only specify the end event
or the start event.
For example:

```
til "half-life 3 time to release" start:"first person to think about half-life 3" 
```

This creates a range with a start date but no end date.

It is also possible to render the timeline on the terminal:

```
til render
```

Although something like a web app would be a much better way to interact
with and visualise timelines, this provides a quick way to look at them.

## .til file

The `.til` file is a binary file that stores data for a single timeline.
