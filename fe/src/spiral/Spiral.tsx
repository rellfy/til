import { useEffect, useMemo, useRef } from "react";
import type { Timeline } from "../lib/timeline";
import { arcPath, parseDateTime, pointAtTheta, spiralPath, thetaFromUnit } from "./math";
import { rangeColor } from "./colors";
import "./Spiral.css";

type EventNode = {
  id: string;
  label: string;
  datetime: string;
  theta: number;
  x: number;
  y: number;
  tagLabels: string[];
};

type RangeArc = {
  id: string;
  label: string;
  d: string;
  color: string;
  tagLabels: string[];
};

type Layout = {
  events: EventNode[];
  ranges: RangeArc[];
  spiralD: string;
  tMin: number;
  tMax: number;
};

// Visible window size around the focal point, in spiral user units.
// Must stay below the radial gap between adjacent turns
// (= (rEnd - rStart) / turns) to never reveal another arm.
const FOCAL_VIEW_WIDTH = 600;

const RANGE_OFFSET = 40;
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
    const theta = thetaFromUnit(unit(parseDateTime(e.datetime)));
    const { x, y } = pointAtTheta(theta);
    return {
      id: e.id,
      label: e.label,
      datetime: e.datetime,
      theta,
      x,
      y,
      tagLabels: tagLabels(e.tags),
    };
  });
  eventNodes.sort((a, b) => a.theta - b.theta);

  const rangeArcs: RangeArc[] = ranges.map((r, i) => {
    const v = r.value;
    let thetaA: number;
    let thetaB: number;
    if ("StartEnd" in v) {
      thetaA = thetaFromUnit(unit(parseDateTime(v.StartEnd[0])));
      thetaB = thetaFromUnit(unit(parseDateTime(v.StartEnd[1])));
    } else if ("Start" in v) {
      thetaA = thetaFromUnit(unit(parseDateTime(v.Start)));
      thetaB = thetaFromUnit(1);
    } else {
      thetaA = thetaFromUnit(0);
      thetaB = thetaFromUnit(unit(parseDateTime(v.End)));
    }
    return {
      id: r.id,
      label: r.label,
      d: arcPath(thetaA, thetaB, RANGE_OFFSET + (i % 4) * 14),
      color: rangeColor(i),
      tagLabels: tagLabels(r.tags),
    };
  });

  return { events: eventNodes, ranges: rangeArcs, spiralD: spiralPath(), tMin, tMax };
}

function formatYear(ms: number): string {
  return new Date(ms).getUTCFullYear().toString();
}

type Props = {
  timeline: Timeline;
};

function Spiral({ timeline }: Props) {
  const svgRef = useRef<SVGSVGElement>(null);
  const layout = useMemo(() => computeLayout(timeline), [timeline]);

  useEffect(() => {
    const svg = svgRef.current;
    if (!svg) return;

    let target = 1;
    let current = 1;
    let rafId = 0;

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

    const onResize = () => updateView();

    svg.addEventListener("wheel", onWheel, { passive: false });
    window.addEventListener("resize", onResize);
    updateView();

    return () => {
      svg.removeEventListener("wheel", onWheel);
      window.removeEventListener("resize", onResize);
      if (rafId) cancelAnimationFrame(rafId);
    };
  }, [layout]);

  return (
    <div className="spiral-host">
      <svg ref={svgRef} className="spiral-svg" preserveAspectRatio="xMidYMid meet">
        <path className="spiral-track" d={layout.spiralD} />
        {layout.ranges.map((r) => (
          <g key={r.id} className="spiral-range">
            <path d={r.d} stroke={r.color} fill="none" strokeLinecap="round" />
            <title>
              {r.label}
              {r.tagLabels.length ? ` [${r.tagLabels.join(", ")}]` : ""}
            </title>
          </g>
        ))}
        {layout.events.map((e) => {
          const out = Math.cos(e.theta) >= 0;
          const offset = 14;
          const tx = e.x + Math.cos(e.theta) * offset;
          const ty = e.y + Math.sin(e.theta) * offset;
          const anchor = out ? "start" : "end";
          const tagSuffix = e.tagLabels.length ? ` [${e.tagLabels.join(", ")}]` : "";
          return (
            <g key={e.id} className="spiral-event">
              <circle cx={e.x} cy={e.y} />
              <text x={tx} y={ty} textAnchor={anchor} dominantBaseline="middle">
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
        <strong>{timeline.label}</strong>
        <span>
          {formatYear(layout.tMin)} - {formatYear(layout.tMax)}
        </span>
        <span>scroll to travel through time</span>
      </div>
    </div>
  );
}

export default Spiral;
