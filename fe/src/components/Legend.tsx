import type {ReactNode} from "react";
import {formatYear} from "../lib/dates";
import {computeTimelineRange} from "../lib/timeline-info";
import type {Timeline} from "../lib/timeline";
import "./Legend.css";

type Props = {
  timeline: Timeline;
  description: string;
  button?: ReactNode;
};

const Legend = ({timeline, description, button}: Props) => {
  const range = computeTimelineRange(timeline);
  return (
    <div className="app-legend">
      <div className="app-legend-title-row">
        <div className="app-legend-title">{timeline.label}</div>
        {button}
      </div>
      <div className="app-legend-range">
        {range ? `${formatYear(range.tMin)} – ${formatYear(range.tMax)}` : "—"}
      </div>
      <div className="app-legend-hint">{description}</div>
    </div>
  );
};

export default Legend;
