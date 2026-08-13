import "./CreatureBody.css";
import type { CreatureBodyModule, CreatureStage } from "./creatureClient";
import { ProceduralMemorySeed } from "./ProceduralMemorySeed";

export interface CreatureBodyProps {
  hasCompletionStar?: boolean;
  motionEnabled?: boolean;
  showOrbits?: boolean;
  stage?: CreatureStage;
  bodyModule?: CreatureBodyModule;
}

export function CreatureBody({
  hasCompletionStar = false,
  motionEnabled = true,
  showOrbits = true,
  stage = "seed",
  bodyModule = "memory-seed-egg-v1",
}: CreatureBodyProps) {
  return (
    <span
      aria-hidden="true"
      className={`creature-body${motionEnabled ? "" : " creature-body-still"}`}
    >
      {showOrbits && <span className="orbit orbit-one" />}
      {showOrbits && <span className="orbit orbit-two" />}
      <span
        className="memoryling"
        data-body-module={bodyModule}
        data-stage={stage}
      >
        <ProceduralMemorySeed bodyModule={bodyModule} stage={stage} />
        {hasCompletionStar && (
          <span
            className="memory-mark derived-completion-star"
            data-testid="derived-memory-mark"
          >
            ✦
          </span>
        )}
      </span>
    </span>
  );
}

export default CreatureBody;
