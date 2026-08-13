import { useEffect, useRef, useState } from "react";
import {
  baselineCreatureRenderState,
  baselinePetShellState,
  isValidRevision,
  sanitizeCreatureRenderState,
  sanitizePetShellState,
  type CreatureClient,
  type CreatureRenderState,
  type PetShellState,
} from "./creatureClient";

export function useCreatureRenderState(client: CreatureClient) {
  const [renderState, setRenderState] = useState<CreatureRenderState>(
    baselineCreatureRenderState,
  );
  const [shellState, setShellState] = useState<PetShellState>(baselinePetShellState);
  const [ready, setReady] = useState(false);
  const latestRequest = useRef(0);
  const latestRevision = useRef(baselineCreatureRenderState.revision);
  const inFlightRevisions = useRef(new Set<string>());

  useEffect(() => {
    let active = true;
    let renderUnlisten: (() => void) | undefined;
    let shellUnlisten: (() => void) | undefined;
    let shellEventVersion = 0;

    async function refetchRenderState(requestedTarget?: string) {
      const request = ++latestRequest.current;
      try {
        const next = sanitizeCreatureRenderState(await client.getRenderState());
        if (!active || request !== latestRequest.current) return;
        latestRevision.current = next.revision;
        setRenderState(next);
      } catch {
        // Keep the last safe state. Native error details never enter the UI.
      } finally {
        if (requestedTarget) inFlightRevisions.current.delete(requestedTarget);
        if (active && request === latestRequest.current) setReady(true);
      }
    }

    async function registerRenderListener() {
      try {
        const unlisten = await client.onRenderRevision(({ revision }) => {
          if (
            !isValidRevision(revision) ||
            revision === latestRevision.current ||
            inFlightRevisions.current.has(revision)
          ) {
            return;
          }
          inFlightRevisions.current.add(revision);
          void refetchRenderState(revision);
        });
        if (!active) unlisten();
        else renderUnlisten = unlisten;
      } catch {
        // Initial fetch remains available if event registration fails.
      }
    }

    async function registerShellListener() {
      try {
        const unlisten = await client.onPetShellState((next) => {
          shellEventVersion += 1;
          if (active) setShellState(sanitizePetShellState(next));
        });
        if (!active) unlisten();
        else shellUnlisten = unlisten;
      } catch {
        // Initial fetch remains available if event registration fails.
      }
    }

    async function start() {
      await Promise.allSettled([registerRenderListener(), registerShellListener()]);
      if (!active) return;
      const initialShellEventVersion = shellEventVersion;
      const results = await Promise.allSettled([
        client.getPetShellState(),
        refetchRenderState(),
      ]);
      if (
        active &&
        results[0].status === "fulfilled" &&
        initialShellEventVersion === shellEventVersion
      ) {
        setShellState(sanitizePetShellState(results[0].value));
      }
      if (active) setReady(true);
    }

    void start();
    return () => {
      active = false;
      latestRequest.current += 1;
      renderUnlisten?.();
      shellUnlisten?.();
    };
  }, [client]);

  return { renderState, setRenderState, shellState, setShellState, ready };
}
