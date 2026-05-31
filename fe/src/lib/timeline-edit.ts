import type {Event, EventRange, Range, Tag, Timeline} from "./timeline";

const uid = (): string => crypto.randomUUID();

export const createTimeline = (label: string): Timeline => ({
  id: uid(),
  label,
  events: {},
  ranges: {},
  tags: {},
});

export const setTimelineLabel = (t: Timeline, label: string): Timeline => ({...t, label});

export const addEvent = (t: Timeline, label: string, datetime: string): Timeline => {
  const e: Event = {id: uid(), label, datetime, tags: []};
  return {...t, events: {...t.events, [e.id]: e}};
};

export const updateEvent = (t: Timeline, id: string, patch: Partial<Event>): Timeline => {
  const cur = t.events[id];
  if (!cur) return t;
  return {...t, events: {...t.events, [id]: {...cur, ...patch}}};
};

export const deleteEvent = (t: Timeline, id: string): Timeline => {
  const events = {...t.events};
  delete events[id];
  return {...t, events};
};

export const addRange = (t: Timeline, label: string, value: EventRange): Timeline => {
  const r: Range = {id: uid(), label, value, tags: []};
  return {...t, ranges: {...t.ranges, [r.id]: r}};
};

export const updateRange = (t: Timeline, id: string, patch: Partial<Range>): Timeline => {
  const cur = t.ranges[id];
  if (!cur) return t;
  return {...t, ranges: {...t.ranges, [id]: {...cur, ...patch}}};
};

export const deleteRange = (t: Timeline, id: string): Timeline => {
  const ranges = {...t.ranges};
  delete ranges[id];
  return {...t, ranges};
};

export const addTag = (t: Timeline, label: string): Timeline => {
  const tag: Tag = {id: uid(), label};
  return {...t, tags: {...t.tags, [tag.id]: tag}};
};

export const updateTag = (t: Timeline, id: string, patch: Partial<Tag>): Timeline => {
  const cur = t.tags[id];
  if (!cur) return t;
  return {...t, tags: {...t.tags, [id]: {...cur, ...patch}}};
};

export const deleteTag = (t: Timeline, id: string): Timeline => {
  const tags = {...t.tags};
  delete tags[id];
  const events: Record<string, Event> = {};
  for (const [eid, e] of Object.entries(t.events)) {
    events[eid] = {...e, tags: e.tags.filter((x) => x !== id)};
  }
  const ranges: Record<string, Range> = {};
  for (const [rid, r] of Object.entries(t.ranges)) {
    ranges[rid] = {...r, tags: r.tags.filter((x) => x !== id)};
  }
  return {...t, tags, events, ranges};
};

export const toggleEventTag = (t: Timeline, eventId: string, tagId: string): Timeline => {
  const cur = t.events[eventId];
  if (!cur) return t;
  const tags = cur.tags.includes(tagId)
    ? cur.tags.filter((x) => x !== tagId)
    : [...cur.tags, tagId];
  return updateEvent(t, eventId, {tags});
};

export const toggleRangeTag = (t: Timeline, rangeId: string, tagId: string): Timeline => {
  const cur = t.ranges[rangeId];
  if (!cur) return t;
  const tags = cur.tags.includes(tagId)
    ? cur.tags.filter((x) => x !== tagId)
    : [...cur.tags, tagId];
  return updateRange(t, rangeId, {tags});
};
