import {useNavigate} from "react-router-dom";
import Spiral from "../spiral/Spiral";
import {createTimeline} from "../lib/timeline-edit";
import {confirmIfDirty, useSaveTimeline, useTimeline} from "../lib/timeline-context";

const Home = () => {
  const {timeline, setTimeline, isDirty} = useTimeline();
  const handleSave = useSaveTimeline();
  const navigate = useNavigate();
  if (!timeline) return <p>Loading sample timeline…</p>;
  return (
    <Spiral
      timeline={timeline}
      onTimelineChange={(t) => setTimeline(t, false)}
      menuItems={[
        {
          label: "new",
          onClick: () => {
            if (!confirmIfDirty(isDirty)) return;
            setTimeline(createTimeline("Untitled timeline"), true);
            navigate("/edit");
          },
        },
        {label: "edit", onClick: () => navigate("/edit")},
        {label: "save", onClick: handleSave},
      ]}
    />
  );
};

export default Home;
