import { useEffect, useState } from "react";
import sampleUrl from "../assets/world-music.til?url";
import { loadTimelineFromUrl, type Timeline } from "../lib/timeline";

type LoadState =
  | { kind: "loading" }
  | { kind: "ready"; timeline: Timeline }
  | { kind: "error"; message: string };

function rangeSpan(r: Timeline["ranges"][string]): string {
  const v = r.value;
  if ("StartEnd" in v) return `${v.StartEnd[0]} to ${v.StartEnd[1]}`;
  if ("Start" in v) return `${v.Start} to ...`;
  return `... to ${v.End}`;
}

function tagLabels(timeline: Timeline, tagIds: string[]): string {
  if (tagIds.length === 0) return "";
  const labels = tagIds
    .map((id) => timeline.tags[id]?.label)
    .filter((l): l is string => l !== undefined);
  return labels.length ? ` [${labels.join(", ")}]` : "";
}

function Home() {
  const [state, setState] = useState<LoadState>({ kind: "loading" });

  useEffect(() => {
    let cancelled = false;
    loadTimelineFromUrl(sampleUrl)
      .then((timeline) => {
        if (!cancelled) setState({ kind: "ready", timeline });
      })
      .catch((err: unknown) => {
        if (!cancelled) {
          setState({ kind: "error", message: err instanceof Error ? err.message : String(err) });
        }
      });
    return () => {
      cancelled = true;
    };
  }, []);

  if (state.kind === "loading") return <p>Loading sample timeline...</p>;
  if (state.kind === "error") return <p>Error: {state.message}</p>;

  const { timeline } = state;
  const events = Object.values(timeline.events).sort((a, b) =>
    a.datetime.localeCompare(b.datetime),
  );
  const ranges = Object.values(timeline.ranges);
  const tags = Object.values(timeline.tags);

  return (
    <section>
      <h1>{timeline.label}</h1>
      <p>
        {events.length} events, {ranges.length} ranges, {tags.length} tags.
      </p>

      <h2>Events</h2>
      <ul>
        {events.map((e) => (
          <li key={e.id}>
            <code>{e.datetime}</code> {e.label}
            {tagLabels(timeline, e.tags)}
          </li>
        ))}
      </ul>

      <h2>Ranges</h2>
      <ul>
        {ranges.map((r) => (
          <li key={r.id}>
            <code>{rangeSpan(r)}</code> {r.label}
            {tagLabels(timeline, r.tags)}
          </li>
        ))}
      </ul>

      <h2>Tags</h2>
      <p>{tags.map((t) => t.label).join(", ")}</p>
    </section>
  );
}

export default Home;
