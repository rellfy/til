import { Route, Routes } from "react-router-dom";
import Editor from "./routes/Editor";
import Home from "./routes/Home";
import NotFound from "./routes/NotFound";
import { TimelineProvider } from "./lib/timeline-context";
import "./App.css";

const App = () => {
  return (
    <div className="app">
      <main>
        <TimelineProvider>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/edit" element={<Editor />} />
            <Route path="*" element={<NotFound />} />
          </Routes>
        </TimelineProvider>
      </main>
    </div>
  );
};

export default App;
