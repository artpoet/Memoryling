import "./CreatureBody.css";

export interface CreatureBodyProps {
  hasCompletionStar?: boolean;
  motionEnabled?: boolean;
  showOrbits?: boolean;
}

export function CreatureBody({
  hasCompletionStar = false,
  motionEnabled = true,
  showOrbits = true,
}: CreatureBodyProps) {
  return (
    <span
      aria-hidden="true"
      className={`creature-body${motionEnabled ? "" : " creature-body-still"}`}
    >
      {showOrbits && <span className="orbit orbit-one" />}
      {showOrbits && <span className="orbit orbit-two" />}
      <span className="memoryling">
        <span className="ear ear-left" />
        <span className="ear ear-right" />
        <span className="face">
          <span className="eye eye-left" />
          <span className="eye eye-right" />
          <span className="cheek cheek-left" />
          <span className="cheek cheek-right" />
          <span className="mouth" />
        </span>
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
