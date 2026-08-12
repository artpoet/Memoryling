import { lazy, Suspense } from "react";
import PetSurface from "./PetSurface";
import type {
  CreatureClient,
  DetailEventClient,
  DetailShellClient,
} from "./creatureClient";
import type { MemoryClient } from "./memoryClient";
import type { AppSurface } from "./surface";
import "./SurfaceRouter.css";

const DetailSurface = lazy(() =>
  import("./App").then((module) => ({ default: module.DetailSurface })),
);

export interface SurfaceRouterProps {
  surface: AppSurface;
  creatureClient?: CreatureClient;
  memoryClient?: MemoryClient;
  detailEvents?: DetailEventClient;
  detailShell?: DetailShellClient;
}

export function SurfaceRouter({
  surface,
  creatureClient,
  memoryClient,
  detailEvents,
  detailShell,
}: SurfaceRouterProps) {
  if (surface === "pet") return <PetSurface client={creatureClient} />;
  if (surface === "detail" || surface === "browser") {
    return (
      <Suspense
        fallback={<div className="surface-loading" role="status">Memoryling</div>}
      >
        <DetailSurface
          browserPreview={surface === "browser"}
          detailEvents={detailEvents}
          detailShell={detailShell}
          memoryClient={memoryClient}
        />
      </Suspense>
    );
  }
  return (
    <main className="unsupported-surface" role="alert">
      <h1>Memoryling</h1>
      <p>This desktop surface is not authorized. Use the installed shortcut or system tray.</p>
      <p lang="zh-TW">這個桌面表面未獲授權；請使用已安裝的捷徑或系統匣。</p>
    </main>
  );
}

export default SurfaceRouter;
