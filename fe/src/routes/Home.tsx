import { useEffect, useState } from "react";
import sampleUrl from "../assets/world-music.til?url";
import { loadTimelineFromUrl, type Timeline } from "../lib/timeline";
import Spiral from "../spiral/Spiral";

type LoadState =
  | { kind: "loading" }
  | { kind: "ready"; timeline: Timeline }
  | { kind: "error"; message: string };

const Home = () => {
  const [state, setState] = useState<LoadState>({ kind: "loading" });

  useEffect(() => {
    let cancelled = false;
    loadTimelineFromUrl(sampleUrl)
      .then((timeline) => {
        if (!cancelled) setState({ kind: "ready", timeline });
      })
      .catch((err: unknown) => {
        if (!cancelled) {
          setState({ kind: "error", message: err instanceof Error ? err.message : String(err) });
        }
      });
    return () => {
      cancelled = true;
    };
  }, []);

  if (state.kind === "loading") return <p>Loading sample timeline...</p>;
  if (state.kind === "error") return <p>Error: {state.message}</p>;

  return (
    <Spiral
      timeline={state.timeline}
      onTimelineChange={(timeline) => setState({kind: "ready", timeline})}
    />
  );
};

export default Home;
