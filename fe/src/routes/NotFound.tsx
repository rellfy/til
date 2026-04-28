import { Link } from "react-router-dom";

function NotFound() {
  return (
    <section>
      <h1>404</h1>
      <p>Nothing here.</p>
      <Link to="/">Back home</Link>
    </section>
  );
}

export default NotFound;
