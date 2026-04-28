import init, { parse_timeline } from "../wasm/til_wasm";

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
};

export type EventRange = { StartEnd: [string, string] } | { Start: string } | { End: string };

export type Range = {
  id: Uuid;
  value: EventRange;
  tags: Uuid[];
  label: string;
};

export type Timeline = {
  id: Uuid;
  events: Record<Uuid, Event>;
  ranges: Record<Uuid, Range>;
  tags: Record<Uuid, Tag>;
  label: string;
};

let wasmReady: Promise<void> | undefined;

function ensureWasm(): Promise<void> {
  if (!wasmReady) {
    wasmReady = init().then(() => undefined);
  }
  return wasmReady;
}

export async function parseTimeline(bytes: Uint8Array): Promise<Timeline> {
  await ensureWasm();
  return parse_timeline(bytes) as Timeline;
}

export async function loadTimelineFromUrl(url: string): Promise<Timeline> {
  const res = await fetch(url);
  if (!res.ok) {
    throw new Error(`failed to fetch timeline: ${res.status}`);
  }
  const bytes = new Uint8Array(await res.arrayBuffer());
  return parseTimeline(bytes);
}
