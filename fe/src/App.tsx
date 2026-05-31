import { Route, Routes } from "react-router-dom";
import Home from "./routes/Home";
import NotFound from "./routes/NotFound";
import "./App.css";

const App = () => {
  return (
    <div className="app">
      <main>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="*" element={<NotFound />} />
        </Routes>
      </main>
    </div>
  );
};

export default App;
