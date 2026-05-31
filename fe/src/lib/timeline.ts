import init, { parse_timeline, serialize_timeline } from "../wasm/til_wasm";

export type Uuid = string;

export type Tag = {
  id: Uuid;
  label: string;
};

export type Event = {
  id: Uuid;
  datetime: string;
  tags: Uuid[];
  label: string;
  ref?: string | undefined;
  attributes?: string | undefined;
};

export type EventRange = { StartEnd: [string, string] } | { Start: string } | { End: string };

export type Range = {
  id: Uuid;
  value: EventRange;
  tags: Uuid[];
  label: string;
  ref?: string | undefined;
  attributes?: string | undefined;
};

export type Timeline = {
  id: Uuid;
  events: Record<Uuid, Event>;
  ranges: Record<Uuid, Range>;
  tags: Record<Uuid, Tag>;
  label: string;
};

let wasmReady: Promise<void> | undefined;

const ensureWasm = (): Promise<void> => {
  if (!wasmReady) {
    wasmReady = init().then(() => undefined);
  }
  return wasmReady;
};

export const parseTimeline = async (bytes: Uint8Array): Promise<Timeline> => {
  await ensureWasm();
  return parse_timeline(bytes) as Timeline;
};

export const loadTimelineFromUrl = async (url: string): Promise<Timeline> => {
  const res = await fetch(url);
  if (!res.ok) {
    throw new Error(`failed to fetch timeline: ${res.status}`);
  }
  const bytes = new Uint8Array(await res.arrayBuffer());
  return parseTimeline(bytes);
};

export const serializeTimeline = async (timeline: Timeline): Promise<Uint8Array> => {
  await ensureWasm();
  return serialize_timeline(timeline);
};

export const downloadTimeline = async (timeline: Timeline): Promise<void> => {
  const bytes = await serializeTimeline(timeline);
  const blob = new Blob([new Uint8Array(bytes)], {type: "application/octet-stream"});
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `${timeline.label || "timeline"}.til`;
  a.click();
  URL.revokeObjectURL(url);
};
