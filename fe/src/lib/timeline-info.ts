import {tryParseToMs} from "./dates";
import type {Timeline} from "./timeline";

export type TimelineRange = {
  tMin: number;
  tMax: number;
};

export const computeTimelineRange = (timeline: Timeline): TimelineRange | null => {
  const times: number[] = [];
  for (const e of Object.values(timeline.events)) {
    const t = tryParseToMs(e.datetime);
    if (t !== null) times.push(t);
  }
  for (const r of Object.values(timeline.ranges)) {
    const v = r.value;
    if ("StartEnd" in v) {
      const a = tryParseToMs(v.StartEnd[0]);
      const b = tryParseToMs(v.StartEnd[1]);
      if (a !== null) times.push(a);
      if (b !== null) times.push(b);
    } else if ("Start" in v) {
      const a = tryParseToMs(v.Start);
      if (a !== null) times.push(a);
    } else {
      const b = tryParseToMs(v.End);
      if (b !== null) times.push(b);
    }
  }
  if (times.length === 0) return null;
  return {tMin: Math.min(...times), tMax: Math.max(...times)};
};
