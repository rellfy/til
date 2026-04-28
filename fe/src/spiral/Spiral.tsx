import { useEffect, useMemo, useRef } from "react";
import type { Timeline } from "../lib/timeline";
import {
  arcPath,
  parseDateTime,
  pointAtTheta,
  radiusAtTheta,
  spiralPath,
  thetaFromUnit,
} from "./math";
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

const RANGE_OFFSET = 40;
const ZOOM_FACTOR = 1.0;
const SCROLL_SENSITIVITY = 0.0008;

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

function clamp(v: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, v));
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
    const sRef = { current: 1 };

    const updateView = () => {
      const s = sRef.current;
      const theta = thetaFromUnit(s);
      const focal = pointAtTheta(theta);
      const r = radiusAtTheta(theta);
      const rect = svg.getBoundingClientRect();
      const pxWidth = rect.width || 800;
      const pxHeight = rect.height || 600;
      const zoom = pxWidth / (2 * r * ZOOM_FACTOR);
      const vw = pxWidth / zoom;
      const vh = pxHeight / zoom;
      svg.setAttribute("viewBox", `${focal.x - vw / 2} ${focal.y - vh / 2} ${vw} ${vh}`);
      svg.style.setProperty("--screen-scale", String(1 / zoom));
    };

    const onWheel = (event: WheelEvent) => {
      event.preventDefault();
      sRef.current = clamp(sRef.current + event.deltaY * SCROLL_SENSITIVITY, 0, 1);
      updateView();
    };

    const onResize = () => updateView();

    svg.addEventListener("wheel", onWheel, { passive: false });
    window.addEventListener("resize", onResize);
    updateView();

    return () => {
      svg.removeEventListener("wheel", onWheel);
      window.removeEventListener("resize", onResize);
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
          const offset = 12;
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
