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
        <a
          className="app-legend-github"
          href="https://github.com/rellfy/til"
          target="_blank"
          rel="noopener noreferrer"
          aria-label="GitHub repository"
          title="rellfy/til on GitHub"
        >
          <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true">
            <path
              fill="currentColor"
              d="M12 .5C5.65.5.5 5.65.5 12c0 5.08 3.29 9.39 7.86 10.91.58.11.79-.25.79-.55v-2.13c-3.2.7-3.88-1.36-3.88-1.36-.53-1.34-1.29-1.7-1.29-1.7-1.05-.72.08-.7.08-.7 1.16.08 1.77 1.19 1.77 1.19 1.03 1.77 2.7 1.26 3.36.96.1-.75.4-1.26.73-1.55-2.55-.29-5.24-1.28-5.24-5.67 0-1.25.45-2.27 1.18-3.07-.12-.29-.51-1.45.11-3.02 0 0 .96-.31 3.15 1.17.91-.25 1.89-.38 2.86-.39.97 0 1.95.14 2.86.39 2.19-1.48 3.15-1.17 3.15-1.17.62 1.57.23 2.73.12 3.02.73.8 1.18 1.82 1.18 3.07 0 4.4-2.69 5.37-5.25 5.66.41.36.78 1.06.78 2.13v3.16c0 .31.21.66.8.55C20.71 21.39 24 17.08 24 12 24 5.65 18.85.5 12 .5Z"
            />
          </svg>
        </a>
      </div>
      <div className="app-legend-range">
        {range ? `${formatYear(range.tMin)} – ${formatYear(range.tMax)}` : "—"}
      </div>
      <div className="app-legend-hint">{description}</div>
    </div>
  );
};

export default Legend;
