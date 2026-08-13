import { useId } from "react";
import type { CreatureBodyModule, CreatureStage } from "./creatureClient";

interface ProceduralMemorySeedProps {
  bodyModule: CreatureBodyModule;
  stage: CreatureStage;
}

export function ProceduralMemorySeed({
  bodyModule,
  stage,
}: ProceduralMemorySeedProps) {
  const id = useId().replace(/:/g, "");
  const bodyGradient = `${id}-body`;
  const leafGradient = `${id}-leaf`;
  const petalGradient = `${id}-petal`;
  const eyeGradient = `${id}-eye`;
  const coreGradient = `${id}-core`;
  const bodyGlow = `${id}-body-glow`;
  const coreGlow = `${id}-core-glow`;

  return (
    <svg
      aria-hidden="true"
      className="memory-seed-renderer"
      data-body-module={bodyModule}
      data-renderer="procedural-svg-v1"
      data-stage={stage}
      data-testid="memoryling-seed-renderer"
      focusable="false"
      role="presentation"
      viewBox="0 0 240 250"
      xmlns="http://www.w3.org/2000/svg"
    >
      <defs>
        <linearGradient id={bodyGradient} x1="0.22" x2="0.82" y1="0.06" y2="0.94">
          <stop offset="0" stopColor="#b49af4" />
          <stop offset="0.42" stopColor="#8163d0" />
          <stop offset="1" stopColor="#4d368f" />
        </linearGradient>
        <linearGradient id={leafGradient} x1="0.18" x2="0.86" y1="0.08" y2="0.92">
          <stop offset="0" stopColor="#cab5ff" />
          <stop offset="0.55" stopColor="#8e70df" />
          <stop offset="1" stopColor="#6147ad" />
        </linearGradient>
        <linearGradient id={petalGradient} x1="0.5" x2="0.5" y1="0" y2="1">
          <stop offset="0" stopColor="#a98deb" />
          <stop offset="1" stopColor="#5b439f" />
        </linearGradient>
        <radialGradient id={eyeGradient} cx="0.34" cy="0.26" r="0.76">
          <stop offset="0" stopColor="#62559b" />
          <stop offset="0.38" stopColor="#292247" />
          <stop offset="1" stopColor="#100d20" />
        </radialGradient>
        <linearGradient id={coreGradient} x1="0.15" x2="0.85" y1="0.08" y2="0.92">
          <stop offset="0" stopColor="#e7fff8" />
          <stop offset="0.46" stopColor="#8ff0d0" />
          <stop offset="1" stopColor="#3ab28e" />
        </linearGradient>
        <filter id={bodyGlow} height="150%" width="150%" x="-25%" y="-20%">
          <feDropShadow dx="0" dy="10" floodColor="#432a84" floodOpacity="0.3" stdDeviation="9" />
        </filter>
        <filter id={coreGlow} height="220%" width="220%" x="-60%" y="-60%">
          <feGaussianBlur result="blur" stdDeviation="4" />
          <feMerge>
            <feMergeNode in="blur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>

      <ellipse cx="120" cy="225" fill="#4e388d" opacity="0.12" rx="63" ry="11" />

      <g className="seed-sprouts" fill={`url(#${leafGradient})`}>
        <path d="M109 55C93 50 82 35 86 14C102 18 115 34 109 55Z" />
        <path d="M130 51C128 33 139 17 155 13C160 33 148 48 130 51Z" />
        <path d="M102 47C99 34 96 25 90 18" fill="none" opacity="0.34" stroke="#eee7ff" strokeLinecap="round" strokeWidth="2" />
        <path d="M137 45C144 34 149 25 154 17" fill="none" opacity="0.3" stroke="#eee7ff" strokeLinecap="round" strokeWidth="2" />
      </g>

      <g filter={`url(#${bodyGlow})`}>
        <path
          className="seed-shell"
          d="M120 42C84 42 61 72 54 116C47 160 62 205 93 222C109 231 131 231 147 222C178 205 193 160 186 116C179 72 156 42 120 42Z"
          fill={`url(#${bodyGradient})`}
        />
        <path
          d="M83 67C67 86 61 117 64 145"
          fill="none"
          opacity="0.2"
          stroke="#f4efff"
          strokeLinecap="round"
          strokeWidth="7"
        />
        <path
          d="M165 77C179 105 180 144 168 175"
          fill="none"
          opacity="0.13"
          stroke="#2d1c67"
          strokeLinecap="round"
          strokeWidth="5"
        />
      </g>

      <g className="seed-core" filter={`url(#${coreGlow})`}>
        <path d="M120 68L130 80L120 93L110 80Z" fill={`url(#${coreGradient})`} />
        <path d="M120 71L125 80L120 88L115 80Z" fill="#eafff8" opacity="0.55" />
      </g>

      <g className="seed-face">
        <g className="seed-eye seed-eye-left" transform="rotate(-5 94 119)">
          <ellipse cx="94" cy="119" fill={`url(#${eyeGradient})`} rx="10" ry="14" />
          <ellipse cx="91" cy="114" fill="#fff" opacity="0.9" rx="3" ry="4" />
          <circle cx="97" cy="123" fill="#9cf2d8" opacity="0.4" r="1.4" />
        </g>
        <g className="seed-eye seed-eye-right" transform="rotate(5 146 119)">
          <ellipse cx="146" cy="119" fill={`url(#${eyeGradient})`} rx="10" ry="14" />
          <ellipse cx="143" cy="114" fill="#fff" opacity="0.9" rx="3" ry="4" />
          <circle cx="149" cy="123" fill="#9cf2d8" opacity="0.4" r="1.4" />
        </g>
        <path d="M111 144C116 149 124 149 129 144" fill="none" opacity="0.75" stroke="#2a2058" strokeLinecap="round" strokeWidth="2.4" />
      </g>

      <g className="seed-petals" fill={`url(#${petalGradient})`} stroke="#c8b3ff" strokeOpacity="0.18" strokeWidth="1">
        <path d="M66 184C79 173 96 176 108 199C92 203 76 197 66 184Z" />
        <path d="M91 199C102 181 120 178 132 198C122 218 104 219 91 199Z" />
        <path d="M126 199C140 178 158 175 174 184C164 201 144 206 126 199Z" />
        <path d="M104 209C115 198 129 198 139 208C131 223 114 225 104 209Z" opacity="0.92" />
      </g>

      <g fill="#cbb9ff" opacity="0.36">
        <circle cx="78" cy="159" r="2" />
        <circle cx="162" cy="156" r="1.7" />
        <circle cx="151" cy="177" r="1.3" />
      </g>
    </svg>
  );
}

export default ProceduralMemorySeed;
