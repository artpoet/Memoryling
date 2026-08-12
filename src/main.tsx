import React from "react";
import ReactDOM from "react-dom/client";
import SurfaceRouter from "./SurfaceRouter";
import { getCurrentSurface } from "./surface";

const surface = getCurrentSurface();
document.documentElement.dataset.surface = surface === "pet" ? "pet" : "detail";
document.title = "Memoryling";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <SurfaceRouter surface={surface} />
  </React.StrictMode>,
);
