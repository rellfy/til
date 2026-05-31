import {createContext, useCallback, useContext, useEffect, useState, type ReactNode} from "react";
import sampleUrl from "../assets/world-music.til?url";
import {downloadTimeline, loadTimelineFromUrl, type Timeline} from "./timeline";

type TimelineState = {
  timeline: Timeline | null;
  isDirty: boolean;
  setTimeline: (t: Timeline, dirty?: boolean) => void;
  markSaved: () => void;
};

const TimelineCtx = createContext<TimelineState | null>(null);

export const TimelineProvider = ({children}: {children: ReactNode}) => {
  const [timeline, setTimelineState] = useState<Timeline | null>(null);
  const [savedRef, setSavedRef] = useState<Timeline | null>(null);
  useEffect(() => {
    let cancelled = false;
    loadTimelineFromUrl(sampleUrl)
      .then((t) => {
        if (cancelled) return;
        setTimelineState(t);
        setSavedRef(t);
      })
      .catch(() => {
        // Ignore initial load failures; user can upload or create new.
      });
    return () => {
      cancelled = true;
    };
  }, []);
  const setTimeline = useCallback((t: Timeline, dirty: boolean = true) => {
    setTimelineState(t);
    if (!dirty) setSavedRef(t);
  }, []);
  const markSaved = useCallback(() => {
    setSavedRef(timeline);
  }, [timeline]);
  return (
    <TimelineCtx.Provider
      value={{timeline, isDirty: timeline !== savedRef, setTimeline, markSaved}}
    >
      {children}
    </TimelineCtx.Provider>
  );
};

export const useTimeline = (): TimelineState => {
  const ctx = useContext(TimelineCtx);
  if (!ctx) throw new Error("useTimeline must be used inside TimelineProvider");
  return ctx;
};

export const useSaveTimeline = (): (() => Promise<void>) => {
  const {timeline, markSaved} = useTimeline();
  return useCallback(async () => {
    if (!timeline) return;
    try {
      await downloadTimeline(timeline);
      markSaved();
    } catch (err) {
      alert(`Failed to save: ${err instanceof Error ? err.message : String(err)}`);
    }
  }, [timeline, markSaved]);
};

export const confirmIfDirty = (isDirty: boolean): boolean => {
  return !isDirty || confirm("You have unsaved changes. Continue?");
};
