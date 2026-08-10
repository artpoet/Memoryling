import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

document.title = "Memoryling — Your agent memories, alive";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
