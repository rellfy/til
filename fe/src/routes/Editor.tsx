import {useMemo, useRef, useState, type ChangeEvent} from "react";
import {useNavigate} from "react-router-dom";
import Legend from "../components/Legend";
import SplitButton from "../components/SplitButton";
import {displayDateTime, normalizeDateTime, tryParseToMs} from "../lib/dates";
import {parseTimeline, type Event, type EventRange, type Range, type Tag, type Timeline} from "../lib/timeline";
import {confirmIfDirty, useSaveTimeline, useTimeline} from "../lib/timeline-context";
import {
  addEvent,
  addRange,
  addTag,
  deleteEvent,
  deleteRange,
  deleteTag,
  setTimelineLabel,
  toggleEventTag,
  toggleRangeTag,
  updateEvent,
  updateRange,
  updateTag,
} from "../lib/timeline-edit";
import "./Editor.css";

type Tab = "timeline" | "tags";

type Row =
  | {kind: "event"; sortKey: number; event: Event}
  | {kind: "range"; sortKey: number; range: Range};

const Editor = () => {
  const {timeline, setTimeline, isDirty} = useTimeline();
  const handleSave = useSaveTimeline();
  const [tab, setTab] = useState<Tab>("timeline");
  const fileRef = useRef<HTMLInputElement>(null);
  const navigate = useNavigate();
  if (!timeline) return <p style={{padding: "2rem"}}>Loading…</p>;

  const handleUploadClick = () => {
    if (!confirmIfDirty(isDirty)) return;
    fileRef.current?.click();
  };

  const handleFile = async (e: ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    e.target.value = "";
    if (!file) return;
    try {
      const bytes = new Uint8Array(await file.arrayBuffer());
      const t = await parseTimeline(bytes);
      setTimeline(t, false);
      navigate("/");
    } catch (err) {
      alert(`Failed to parse timeline: ${err instanceof Error ? err.message : String(err)}`);
    }
  };

  const update = (next: Timeline) => setTimeline(next, true);

  return (
    <div className="editor-host">
      <Legend
        timeline={timeline}
        description="timeline editor"
        button={
          <SplitButton
            label="save"
            onClick={handleSave}
            items={[
              {label: "upload", onClick: handleUploadClick},
              {label: "view", onClick: () => navigate("/")},
            ]}
          />
        }
      />
      <input
        ref={fileRef}
        type="file"
        accept=".til"
        className="editor-file-input"
        onChange={handleFile}
      />
      <div className="editor-content">
        <div className="editor-title-row">
          <label className="editor-field-label">timeline name</label>
          <input
            className="editor-title-input"
            value={timeline.label}
            onChange={(e) => update(setTimelineLabel(timeline, e.target.value))}
          />
        </div>
        <div className="editor-tabs">
          <button
            type="button"
            className={`editor-tab${tab === "timeline" ? " active" : ""}`}
            onClick={() => setTab("timeline")}
          >
            timeline
          </button>
          <button
            type="button"
            className={`editor-tab${tab === "tags" ? " active" : ""}`}
            onClick={() => setTab("tags")}
          >
            tags
          </button>
        </div>
        {tab === "timeline" ? (
          <TimelineList timeline={timeline} onChange={update} />
        ) : (
          <TagsList timeline={timeline} onChange={update} />
        )}
      </div>
    </div>
  );
};

type ListProps = {
  timeline: Timeline;
  onChange: (next: Timeline) => void;
};

const TimelineList = ({timeline, onChange}: ListProps) => {
  const rows: Row[] = useMemo(() => {
    const out: Row[] = [];
    for (const e of Object.values(timeline.events)) {
      const t = tryParseToMs(e.datetime) ?? Number.POSITIVE_INFINITY;
      out.push({kind: "event", sortKey: t, event: e});
    }
    for (const r of Object.values(timeline.ranges)) {
      const t = rangeSortKey(r.value);
      out.push({kind: "range", sortKey: t, range: r});
    }
    out.sort((a, b) => a.sortKey - b.sortKey);
    return out;
  }, [timeline]);

  return (
    <div className="editor-list">
      {rows.map((row) =>
        row.kind === "event" ? (
          <EventRow
            key={row.event.id}
            event={row.event}
            timeline={timeline}
            onChange={onChange}
          />
        ) : (
          <RangeRow
            key={row.range.id}
            range={row.range}
            timeline={timeline}
            onChange={onChange}
          />
        ),
      )}
      <div className="editor-add-row">
        <button
          type="button"
          className="editor-add-button"
          onClick={() => onChange(addEvent(timeline, "new event", todayIso()))}
        >
          + event
        </button>
        <button
          type="button"
          className="editor-add-button"
          onClick={() =>
            onChange(
              addRange(timeline, "new range", {StartEnd: [todayIso(), todayIso()]}),
            )
          }
        >
          + range
        </button>
      </div>
    </div>
  );
};

type EventRowProps = {
  event: Event;
  timeline: Timeline;
  onChange: (next: Timeline) => void;
};

const EventRow = ({event, timeline, onChange}: EventRowProps) => {
  const [forceRef, setForceRef] = useState(false);
  const showRef = !!event.ref || forceRef;
  const setRef = (v: string) =>
    onChange(updateEvent(timeline, event.id, {ref: v || undefined}));
  return (
    <div className="editor-row-wrap">
      <div className="editor-row editor-row-event">
        <div className="editor-row-type">event</div>
        <DateField
          value={event.datetime}
          onCommit={(v) => onChange(updateEvent(timeline, event.id, {datetime: v}))}
        />
        <input
          className="editor-label-input"
          value={event.label}
          onChange={(e) => onChange(updateEvent(timeline, event.id, {label: e.target.value}))}
        />
        <TagSelector
          selected={event.tags}
          tags={timeline.tags}
          onToggle={(tagId) => onChange(toggleEventTag(timeline, event.id, tagId))}
        />
        {!showRef && (
          <button
            type="button"
            className="editor-ref-add"
            onClick={() => setForceRef(true)}
          >
            + ref
          </button>
        )}
        <button
          type="button"
          className="editor-delete"
          onClick={() => onChange(deleteEvent(timeline, event.id))}
          aria-label="delete"
        >
          ×
        </button>
      </div>
      {showRef && (
        <RefRow
          value={event.ref ?? ""}
          onChange={setRef}
          onBlurEmpty={() => setForceRef(false)}
        />
      )}
    </div>
  );
};

type RangeRowProps = {
  range: Range;
  timeline: Timeline;
  onChange: (next: Timeline) => void;
};

const RangeRow = ({range, timeline, onChange}: RangeRowProps) => {
  const [forceRef, setForceRef] = useState(false);
  const showRef = !!range.ref || forceRef;
  const variant: "StartEnd" | "Start" | "End" =
    "StartEnd" in range.value ? "StartEnd" : "Start" in range.value ? "Start" : "End";
  const startVal = "StartEnd" in range.value
    ? range.value.StartEnd[0]
    : "Start" in range.value
      ? range.value.Start
      : "";
  const endVal = "StartEnd" in range.value
    ? range.value.StartEnd[1]
    : "End" in range.value
      ? range.value.End
      : "";

  const setVariant = (next: "StartEnd" | "Start" | "End") => {
    const today = todayIso();
    const nextVal: EventRange =
      next === "StartEnd"
        ? {StartEnd: [startVal || today, endVal || today]}
        : next === "Start"
          ? {Start: startVal || today}
          : {End: endVal || today};
    onChange(updateRange(timeline, range.id, {value: nextVal}));
  };
  const setStart = (v: string) => {
    const nextVal: EventRange =
      variant === "StartEnd" ? {StartEnd: [v, endVal]} : {Start: v};
    onChange(updateRange(timeline, range.id, {value: nextVal}));
  };
  const setEnd = (v: string) => {
    const nextVal: EventRange =
      variant === "StartEnd" ? {StartEnd: [startVal, v]} : {End: v};
    onChange(updateRange(timeline, range.id, {value: nextVal}));
  };
  const setRef = (v: string) =>
    onChange(updateRange(timeline, range.id, {ref: v || undefined}));

  return (
    <div className="editor-row-wrap">
      <div className="editor-row editor-row-range">
        <div className="editor-row-type">range</div>
        <select
          className="editor-variant-select"
          value={variant}
          onChange={(e) => setVariant(e.target.value as "StartEnd" | "Start" | "End")}
        >
          <option value="StartEnd">start–end</option>
          <option value="Start">start only</option>
          <option value="End">end only</option>
        </select>
        {variant !== "End" ? (
          <DateField value={startVal} onCommit={setStart} placeholder="start" />
        ) : (
          <div className="editor-date-placeholder">—</div>
        )}
        {variant !== "Start" ? (
          <DateField value={endVal} onCommit={setEnd} placeholder="end" />
        ) : (
          <div className="editor-date-placeholder">—</div>
        )}
        <input
          className="editor-label-input"
          value={range.label}
          onChange={(e) => onChange(updateRange(timeline, range.id, {label: e.target.value}))}
        />
        <TagSelector
          selected={range.tags}
          tags={timeline.tags}
          onToggle={(tagId) => onChange(toggleRangeTag(timeline, range.id, tagId))}
        />
        {!showRef && (
          <button
            type="button"
            className="editor-ref-add"
            onClick={() => setForceRef(true)}
          >
            + ref
          </button>
        )}
        <button
          type="button"
          className="editor-delete"
          onClick={() => onChange(deleteRange(timeline, range.id))}
          aria-label="delete"
        >
          ×
        </button>
      </div>
      {showRef && (
        <RefRow
          value={range.ref ?? ""}
          onChange={setRef}
          onBlurEmpty={() => setForceRef(false)}
        />
      )}
    </div>
  );
};

type RefRowProps = {
  value: string;
  onChange: (v: string) => void;
  onBlurEmpty?: () => void;
};

const RefRow = ({value, onChange, onBlurEmpty}: RefRowProps) => {
  const isUrl = /^https?:\/\//i.test(value);
  return (
    <div className="editor-ref-row">
      <span className="editor-ref-row-label">ref</span>
      <input
        className="editor-ref-row-input"
        value={value}
        autoFocus={!value}
        placeholder="https://… or any identifier"
        onChange={(e) => onChange(e.target.value)}
        onBlur={(e) => {
          if (!e.target.value) onBlurEmpty?.();
        }}
      />
      {isUrl && (
        <a
          href={value}
          target="_blank"
          rel="noopener noreferrer"
          className="editor-ref-row-open"
          title="open in new tab"
        >
          ↗
        </a>
      )}
    </div>
  );
};

type DateFieldProps = {
  value: string;
  onCommit: (v: string) => void;
  placeholder?: string;
};

const DateField = ({value, onCommit, placeholder}: DateFieldProps) => {
  const [draft, setDraft] = useState(displayDateTime(value));
  const [invalid, setInvalid] = useState(false);
  // Keep draft in sync with prop when outer state changes (e.g., variant switch).
  const lastValue = useRef(value);
  if (lastValue.current !== value) {
    lastValue.current = value;
    setDraft(displayDateTime(value));
    setInvalid(false);
  }
  const commit = () => {
    const normalized = normalizeDateTime(draft);
    if (normalized === null) {
      setInvalid(true);
      return;
    }
    setInvalid(false);
    if (normalized !== value) onCommit(normalized);
  };
  return (
    <input
      className={`editor-date-input${invalid ? " invalid" : ""}`}
      value={draft}
      placeholder={placeholder}
      onChange={(e) => setDraft(e.target.value)}
      onBlur={commit}
      onKeyDown={(e) => {
        if (e.key === "Enter") (e.target as HTMLInputElement).blur();
      }}
    />
  );
};

type TagSelectorProps = {
  selected: string[];
  tags: Record<string, Tag>;
  onToggle: (tagId: string) => void;
};

const TagSelector = ({selected, tags, onToggle}: TagSelectorProps) => {
  const [open, setOpen] = useState(false);
  const selectedSet = new Set(selected);
  const all = Object.values(tags).sort((a, b) => a.label.localeCompare(b.label));
  return (
    <div className="editor-tag-cell">
      <div className="editor-tag-chips">
        {selected.map((tid) => {
          const tag = tags[tid];
          if (!tag) return null;
          return (
            <button
              key={tid}
              type="button"
              className="editor-tag-chip"
              onClick={() => onToggle(tid)}
              title="remove"
            >
              {tag.label} ×
            </button>
          );
        })}
        <button
          type="button"
          className="editor-tag-add"
          onClick={() => setOpen((o) => !o)}
        >
          + tag
        </button>
      </div>
      {open && (
        <div className="editor-tag-menu">
          {all.length === 0 && <div className="editor-tag-empty">no tags yet</div>}
          {all.map((tag) => (
            <label key={tag.id} className="editor-tag-option">
              <input
                type="checkbox"
                checked={selectedSet.has(tag.id)}
                onChange={() => onToggle(tag.id)}
              />
              {tag.label}
            </label>
          ))}
        </div>
      )}
    </div>
  );
};

const TagsList = ({timeline, onChange}: ListProps) => {
  const tags = Object.values(timeline.tags).sort((a, b) =>
    a.label.localeCompare(b.label),
  );
  return (
    <div className="editor-list">
      {tags.map((tag) => (
        <div key={tag.id} className="editor-row editor-row-tag">
          <div className="editor-row-type">tag</div>
          <input
            className="editor-label-input"
            value={tag.label}
            onChange={(e) => onChange(updateTag(timeline, tag.id, {label: e.target.value}))}
          />
          <button
            type="button"
            className="editor-delete"
            onClick={() => onChange(deleteTag(timeline, tag.id))}
            aria-label="delete"
          >
            ×
          </button>
        </div>
      ))}
      <div className="editor-add-row">
        <button
          type="button"
          className="editor-add-button"
          onClick={() => onChange(addTag(timeline, "new tag"))}
        >
          + tag
        </button>
      </div>
    </div>
  );
};

const rangeSortKey = (value: EventRange): number => {
  if ("StartEnd" in value) return tryParseToMs(value.StartEnd[0]) ?? Number.POSITIVE_INFINITY;
  if ("Start" in value) return tryParseToMs(value.Start) ?? Number.POSITIVE_INFINITY;
  return tryParseToMs(value.End) ?? Number.POSITIVE_INFINITY;
};

const todayIso = (): string => {
  const d = new Date();
  const y = String(d.getUTCFullYear()).padStart(4, "0");
  const m = String(d.getUTCMonth() + 1).padStart(2, "0");
  const day = String(d.getUTCDate()).padStart(2, "0");
  return `${y}-${m}-${day}T00:00:00`;
};

export default Editor;
