import {useEffect, useMemo, useRef} from "react";
import type {Timeline} from "../lib/timeline";
import {
  arcPath,
  parseDateTime,
  pointAtTheta,
  pointAtThetaOffset,
  radiusAtTheta,
  spiralPath,
  thetaFromUnit,
} from "./math";
import {rangeColor} from "./colors";
import "./Spiral.css";

type EventNode = {
  id: string;
  label: string;
  datetime: string;
  unit: number;
  theta: number;
  x: number;
  y: number;
  labelX: number;
  labelY: number;
  labelOut: boolean;
  tagLabels: string[];
};

type RangeSegment = {
  d: string;
  strokeWidth: number;
};

type RangeArc = {
  id: string;
  label: string;
  color: string;
  segments: RangeSegment[];
  startTheta: number;
  endTheta: number;
  layer: number;
  tagLabels: string[];
};

type Layout = {
  events: EventNode[];
  ranges: RangeArc[];
  spiralD: string;
  tMin: number;
  tMax: number;
  maxLabelChars: number;
};

// Visible window size around the focal point, in spiral user units.
// Must stay below the radial gap between adjacent turns
// (= (rEnd - rStart) / turns) to never reveal another arm.
const FOCAL_VIEW_WIDTH = 380;

const RANGE_STROKE_BASE = 6;
const RANGE_STROKE_STRIDE = 8;
const LABEL_INWARD_BASE_PX = 10;
const LABEL_INWARD_PER_CHAR_PX = 4;
const LABEL_LAYER_HEIGHT_PX = 20;
const RANGE_LABEL_BOUNDARY_PADDING_PX = 35;

const EVENT_LABEL_CHAR_WIDTH_USER = 3.7;
const EVENT_LABEL_HEIGHT_USER = 7;
const EVENT_LABEL_GAP_USER = 3;
const EVENT_LABEL_SHIFT_STEP_USER = 10;
const EVENT_LABEL_MAX_STEPS = 40;

const MONTH_NAMES = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun",
  "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

function formatDate(ms: number): string {
  const d = new Date(ms);
  return `${d.getUTCDate()} ${MONTH_NAMES[d.getUTCMonth()]} ${d.getUTCFullYear()}`;
}

function tangentRotationDeg(t: number): number {
  const deg = ((((t * 180) / Math.PI + 90) % 360) + 360) % 360;
  return deg > 90 && deg < 270 ? deg + 180 : deg;
}

const SCROLL_SENSITIVITY = 0.00015;
const SCROLL_EASE = 0.18;
const SCROLL_EPSILON = 0.0002;

const WHEEL_LINE_PX = 16;
const WHEEL_PAGE_PX = 800;

function normalizeWheelDelta(event: WheelEvent): number {
  let dy = event.deltaY;
  if (event.deltaMode === 1) dy *= WHEEL_LINE_PX;
  else if (event.deltaMode === 2) dy *= WHEEL_PAGE_PX;
  return dy;
}

function clamp(v: number, lo: number, hi: number): number {
  return Math.max(lo, Math.min(hi, v));
}

function placeEventLabels(eventNodes: EventNode[]): void {
  type LabelBox = { left: number; right: number; top: number; bottom: number };
  const placed: LabelBox[] = [];
  for (const e of eventNodes) {
    const tagSuffix = e.tagLabels.length ? ` [${e.tagLabels.join(", ")}]` : "";
    const W = (e.label + tagSuffix).length * EVENT_LABEL_CHAR_WIDTH_USER;
    const H = EVENT_LABEL_HEIGHT_USER;
    const rEvent = radiusAtTheta(e.theta);
    let shift = 0;
    let box: LabelBox = {left: 0, right: 0, top: 0, bottom: 0};
    let labelX = 0;
    let labelY = 0;
    let labelOut = false;
    for (let step = 0; step <= EVENT_LABEL_MAX_STEPS; step++) {
      const thetaLabel = e.theta + shift / rEvent;
      const p = pointAtThetaOffset(thetaLabel, 14);
      labelX = p.x;
      labelY = p.y;
      labelOut = Math.cos(thetaLabel) >= 0;
      const left = labelOut ? labelX : labelX - W;
      const right = labelOut ? labelX + W : labelX;
      const top = labelY - H / 2;
      const bottom = labelY + H / 2;
      box = {left, right, top, bottom};
      let conflict = false;
      for (const b of placed) {
        if (
          left < b.right + EVENT_LABEL_GAP_USER &&
          right > b.left - EVENT_LABEL_GAP_USER &&
          top < b.bottom + EVENT_LABEL_GAP_USER &&
          bottom > b.top - EVENT_LABEL_GAP_USER
        ) {
          conflict = true;
          break;
        }
      }
      if (!conflict) break;
      shift += EVENT_LABEL_SHIFT_STEP_USER;
    }
    placed.push(box);
    e.labelX = labelX;
    e.labelY = labelY;
    e.labelOut = labelOut;
  }
}

function computeLayout(timeline: Timeline): Layout {
  const events = Object.values(timeline.events);
  const ranges = Object.values(timeline.ranges);

  const allTimes: number[] = [];
  for (const e of events) allTimes.push(parseDateTime(e.datetime));
  for (const r of ranges) {
    if ("StartEnd" in r.value) {
      allTimes.push(parseDateTime(r.value.StartEnd[0]));
      allTimes.push(parseDateTime(r.value.StartEnd[1]));
    } else if ("Start" in r.value) {
      allTimes.push(parseDateTime(r.value.Start));
    } else {
      allTimes.push(parseDateTime(r.value.End));
    }
  }
  const tMin = Math.min(...allTimes);
  const tMax = Math.max(...allTimes);
  const span = tMax - tMin || 1;
  const unit = (t: number) => (t - tMin) / span;

  const tagLabel = (id: string) => timeline.tags[id]?.label;
  const tagLabels = (ids: string[]) =>
    ids.map(tagLabel).filter((l): l is string => l !== undefined);

  const eventNodes: EventNode[] = events.map((e) => {
    const u = unit(parseDateTime(e.datetime));
    const theta = thetaFromUnit(u);
    const {x, y} = pointAtTheta(theta);
    return {
      id: e.id,
      label: e.label,
      datetime: e.datetime,
      unit: u,
      theta,
      x,
      y,
      labelX: 0,
      labelY: 0,
      labelOut: false,
      tagLabels: tagLabels(e.tags),
    };
  });
  eventNodes.sort((a, b) => a.unit - b.unit);
  placeEventLabels(eventNodes);
  const rangeBounds = ranges.map((r) => {
    const v = r.value;
    let a: number;
    let b: number;
    if ("StartEnd" in v) {
      a = thetaFromUnit(unit(parseDateTime(v.StartEnd[0])));
      b = thetaFromUnit(unit(parseDateTime(v.StartEnd[1])));
    } else if ("Start" in v) {
      a = thetaFromUnit(unit(parseDateTime(v.Start)));
      b = thetaFromUnit(1);
    } else {
      a = thetaFromUnit(0);
      b = thetaFromUnit(unit(parseDateTime(v.End)));
    }
    return a <= b ? {start: a, end: b} : {start: b, end: a};
  });
  type LaneEvent = { theta: number; kind: 0 | 1; idx: number };
  const laneEvents: LaneEvent[] = [];
  for (let i = 0; i < ranges.length; i++) {
    laneEvents.push({theta: rangeBounds[i].start, kind: 1, idx: i});
    laneEvents.push({theta: rangeBounds[i].end, kind: 0, idx: i});
  }
  laneEvents.sort((a, b) => a.theta - b.theta || a.kind - b.kind);
  const layerOf = new Array<number>(ranges.length);
  const freeLanes: number[] = [];
  let nextNewLane = 0;
  for (const ev of laneEvents) {
    if (ev.kind === 0) {
      const lane = layerOf[ev.idx];
      let insertAt = freeLanes.length;
      for (let i = 0; i < freeLanes.length; i++) {
        if (freeLanes[i] > lane) {
          insertAt = i;
          break;
        }
      }
      freeLanes.splice(insertAt, 0, lane);
    } else {
      layerOf[ev.idx] = freeLanes.length > 0 ? freeLanes.shift()! : nextNewLane++;
    }
  }
  let maxLabelChars = "31 Dec 9999".length;
  for (const r of ranges) maxLabelChars = Math.max(maxLabelChars, r.label.length);
  const rangeArcs: RangeArc[] = ranges.map((r, i) => {
    const {start, end} = rangeBounds[i];
    const cuts = new Set<number>([start, end]);
    for (let j = 0; j < ranges.length; j++) {
      if (j === i) continue;
      const {start: s, end: e} = rangeBounds[j];
      if (s > start && s < end) cuts.add(s);
      if (e > start && e < end) cuts.add(e);
    }
    const sorted = [...cuts].sort((a, b) => a - b);
    const segments: RangeSegment[] = [];
    for (let k = 0; k < sorted.length - 1; k++) {
      const a = sorted[k];
      const b = sorted[k + 1];
      const mid = (a + b) / 2;
      let count = 0;
      let rank = 0;
      for (let j = 0; j < ranges.length; j++) {
        const rb = rangeBounds[j];
        if (rb.start <= mid && rb.end >= mid) {
          if (j < i) rank++;
          count++;
        }
      }
      const strokeWidth = RANGE_STROKE_BASE + (count - 1 - rank) * RANGE_STROKE_STRIDE;
      segments.push({d: arcPath(a, b), strokeWidth});
    }
    return {
      id: r.id,
      label: r.label,
      color: rangeColor(i),
      segments,
      startTheta: start,
      endTheta: end,
      layer: layerOf[i],
      tagLabels: tagLabels(r.tags),
    };
  });

  return {
    events: eventNodes,
    ranges: rangeArcs,
    spiralD: spiralPath(),
    tMin,
    tMax,
    maxLabelChars,
  };
}

function formatYear(ms: number): string {
  return new Date(ms).getUTCFullYear().toString();
}

type Props = {
  timeline: Timeline;
};

function Spiral({timeline}: Props) {
  const svgRef = useRef<SVGSVGElement>(null);
  const layout = useMemo(() => computeLayout(timeline), [timeline]);

  useEffect(() => {
    const svg = svgRef.current;
    if (!svg) return;

    let target = 0;
    let current = 0;
    let rafId = 0;

    const labelMeta = new Map(
      layout.ranges.map((r) => [
        r.id,
        {startTheta: r.startTheta, endTheta: r.endTheta, layer: r.layer},
      ]),
    );
    const updateView = () => {
      const theta = thetaFromUnit(current);
      const focal = pointAtTheta(theta);
      const rect = svg.getBoundingClientRect();
      const aspect = rect.width === 0 ? 1.5 : rect.width / rect.height;
      const w = FOCAL_VIEW_WIDTH;
      const h = w / aspect;
      svg.setAttribute("viewBox", `${focal.x - w / 2} ${focal.y - h / 2} ${w} ${h}`);
      const screenScale = (rect.width || 1) / w;
      svg.style.setProperty("--screen-scale", String(1 / screenScale));
      const labels = svg.querySelectorAll<SVGTextElement>(".spiral-range-label");
      labels.forEach((el) => {
        const meta = labelMeta.get(el.dataset.rangeId ?? "");
        if (!meta) return;
        const inwardPx =
          LABEL_INWARD_BASE_PX +
          layout.maxLabelChars * LABEL_INWARD_PER_CHAR_PX +
          (meta.layer + 1) * LABEL_LAYER_HEIGHT_PX;
        const inwardUnits = inwardPx / screenScale;
        const halfWidthUnits = el.getBBox().width / 2;
        const paddingUnits = RANGE_LABEL_BOUNDARY_PADDING_PX / screenScale;
        const midTheta = (meta.startTheta + meta.endTheta) / 2;
        const rInward = Math.max(radiusAtTheta(midTheta) - inwardUnits, 1);
        const halfDtheta = (halfWidthUnits + paddingUnits) / rInward;
        const effStart = meta.startTheta + halfDtheta;
        const effEnd = meta.endTheta - halfDtheta;
        const t = effStart >= effEnd ? midTheta : clamp(theta, effStart, effEnd);
        const p = pointAtThetaOffset(t, -inwardUnits);
        el.setAttribute("x", p.x.toFixed(2));
        el.setAttribute("y", p.y.toFixed(2));
        el.setAttribute(
          "transform",
          `rotate(${tangentRotationDeg(t).toFixed(2)} ${p.x.toFixed(2)} ${p.y.toFixed(2)})`,
        );
      });
      const dateEl = svg.querySelector<SVGTextElement>(".spiral-date-label");
      if (dateEl) {
        const dateStr = formatDate(layout.tMin + current * (layout.tMax - layout.tMin));
        dateEl.textContent = dateStr;
        const inwardPx =
          LABEL_INWARD_BASE_PX + layout.maxLabelChars * LABEL_INWARD_PER_CHAR_PX;
        const inwardUnits = inwardPx / screenScale;
        const p = pointAtThetaOffset(theta, -inwardUnits);
        dateEl.setAttribute("x", p.x.toFixed(2));
        dateEl.setAttribute("y", p.y.toFixed(2));
        dateEl.setAttribute(
          "transform",
          `rotate(${tangentRotationDeg(theta).toFixed(2)} ${p.x.toFixed(2)} ${p.y.toFixed(2)})`,
        );
      }
    };

    const animate = () => {
      const diff = target - current;
      if (Math.abs(diff) < SCROLL_EPSILON) {
        current = target;
        updateView();
        rafId = 0;
        return;
      }
      current += diff * SCROLL_EASE;
      updateView();
      rafId = requestAnimationFrame(animate);
    };

    const onWheel = (event: WheelEvent) => {
      event.preventDefault();
      const dy = normalizeWheelDelta(event);
      target = clamp(target + dy * SCROLL_SENSITIVITY, 0, 1);
      if (!rafId) rafId = requestAnimationFrame(animate);
    };

    const onClick = (event: MouseEvent) => {
      const t = event.target as Element | null;
      const group = t?.closest(".spiral-event") as HTMLElement | null;
      if (!group) return;
      const unitStr = group.dataset.eventUnit;
      if (!unitStr) return;
      const u = parseFloat(unitStr);
      if (Number.isNaN(u)) return;
      target = clamp(u, 0, 1);
      if (!rafId) rafId = requestAnimationFrame(animate);
    };

    const onResize = () => updateView();

    svg.addEventListener("wheel", onWheel, {passive: false});
    svg.addEventListener("click", onClick);
    window.addEventListener("resize", onResize);
    updateView();

    return () => {
      svg.removeEventListener("wheel", onWheel);
      svg.removeEventListener("click", onClick);
      window.removeEventListener("resize", onResize);
      if (rafId) cancelAnimationFrame(rafId);
    };
  }, [layout]);

  return (
    <div className="spiral-host">
      <svg ref={svgRef} className="spiral-svg"
           preserveAspectRatio="xMidYMid meet">
        <path className="spiral-track" d={layout.spiralD}/>
        <text className="spiral-date-label" textAnchor="middle"
              dominantBaseline="middle"/>
        {layout.ranges.map((r) => (
          <g key={r.id} className="spiral-range">
            {r.segments.map((s, k) => (
              <path
                key={k}
                d={s.d}
                stroke={r.color}
                strokeWidth={s.strokeWidth}
                fill="none"
                strokeLinecap="round"
              />
            ))}
            <text
              className="spiral-range-label"
              data-range-id={r.id}
              fill={r.color}
              textAnchor="middle"
              dominantBaseline="middle"
            >
              {r.label}
            </text>
            <title>
              {r.label}
              {r.tagLabels.length ? ` [${r.tagLabels.join(", ")}]` : ""}
            </title>
          </g>
        ))}
        {layout.events.map((e) => {
          const anchor = e.labelOut ? "start" : "end";
          const tagSuffix = e.tagLabels.length ? ` [${e.tagLabels.join(", ")}]` : "";
          return (
            <g key={e.id} className="spiral-event" data-event-unit={e.unit}>
              <circle cx={e.x} cy={e.y}/>
              <text x={e.labelX} y={e.labelY} textAnchor={anchor}
                    dominantBaseline="middle">
                {e.label}
                <tspan className="spiral-tags">{tagSuffix}</tspan>
              </text>
              <title>
                {e.label} ({e.datetime}){tagSuffix}
              </title>
            </g>
          );
        })}
      </svg>
      <div className="spiral-legend">
        <div className="spiral-legend-title">{timeline.label}</div>
        <div className="spiral-legend-range">
          {formatYear(layout.tMin)} – {formatYear(layout.tMax)}
        </div>
        <div className="spiral-legend-hint">scroll to travel through time</div>
      </div>
    </div>
  );
}

export default Spiral;
