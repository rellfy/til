import { useEffect, useMemo, useRef } from "react";
import { select } from "d3-selection";
import { zoom, zoomIdentity, type ZoomBehavior } from "d3-zoom";
import type { Timeline } from "../lib/timeline";
import {
  DEFAULT_CONFIG,
  arcPath,
  parseDateTime,
  pointAtTheta,
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
  bounds: { minX: number; minY: number; size: number };
  tMin: number;
  tMax: number;
};

const SVG_PADDING = 200;
const RANGE_OFFSET = 40;
const DOT_RADIUS = 5;

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

  const spiralD = spiralPath();
  const reach = DEFAULT_CONFIG.rEnd + SVG_PADDING + RANGE_OFFSET + 100;
  const bounds = { minX: -reach, minY: -reach, size: reach * 2 };

  return { events: eventNodes, ranges: rangeArcs, spiralD, bounds, tMin, tMax };
}

function formatYear(ms: number): string {
  return new Date(ms).getUTCFullYear().toString();
}

type Props = {
  timeline: Timeline;
};

function Spiral({ timeline }: Props) {
  const svgRef = useRef<SVGSVGElement>(null);
  const viewportRef = useRef<SVGGElement>(null);
  const layout = useMemo(() => computeLayout(timeline), [timeline]);

  useEffect(() => {
    const svgEl = svgRef.current;
    const gEl = viewportRef.current;
    if (!svgEl || !gEl) return;

    const svg = select(svgEl);
    const g = select(gEl);
    const behavior: ZoomBehavior<SVGSVGElement, unknown> = zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.05, 80])
      .on("zoom", (event) => {
        g.attr("transform", event.transform.toString());
      });

    svg.call(behavior);
    svg.call(behavior.transform, zoomIdentity);

    return () => {
      svg.on(".zoom", null);
    };
  }, [layout]);

  const { bounds } = layout;
  const viewBox = `${bounds.minX} ${bounds.minY} ${bounds.size} ${bounds.size}`;

  return (
    <div className="spiral-host">
      <svg
        ref={svgRef}
        className="spiral-svg"
        viewBox={viewBox}
        preserveAspectRatio="xMidYMid meet"
      >
        <g ref={viewportRef}>
          <path className="spiral-track" d={layout.spiralD} />
          {layout.ranges.map((r) => (
            <g key={r.id} className="spiral-range">
              <path d={r.d} stroke={r.color} fill="none" strokeWidth={6} strokeLinecap="round" />
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
                <circle cx={e.x} cy={e.y} r={DOT_RADIUS} />
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
        </g>
      </svg>
      <div className="spiral-legend">
        <strong>{timeline.label}</strong>
        <span>
          {formatYear(layout.tMin)} - {formatYear(layout.tMax)}
        </span>
        <span>scroll to zoom, drag to pan</span>
      </div>
    </div>
  );
}

export default Spiral;
