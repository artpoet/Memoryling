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
      data-renderer="procedural-svg-v3"
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

      <ellipse cx="120" cy="229" fill="#4e388d" opacity="0.12" rx="66" ry="11" />

      <g className="seed-sprouts" fill={`url(#${leafGradient})`}>
        <path d="M108 56C91 52 77 36 79 13C98 16 115 35 108 56Z" />
        <path d="M132 54C130 34 144 15 162 11C165 34 151 51 132 54Z" />
        <path d="M101 48C96 34 89 23 82 16" fill="none" opacity="0.34" stroke="#eee7ff" strokeLinecap="round" strokeWidth="2" />
        <path d="M139 46C148 34 155 22 160 15" fill="none" opacity="0.3" stroke="#eee7ff" strokeLinecap="round" strokeWidth="2" />
      </g>

      <g filter={`url(#${bodyGlow})`}>
        <path
          className="seed-shell"
          d="M120 37C109 49 88 52 72 69C55 87 48 116 49 150C50 188 73 215 103 226C114 230 126 230 137 226C167 215 190 188 191 150C192 116 185 87 168 69C152 52 131 49 120 37Z"
          fill={`url(#${bodyGradient})`}
        />
        <path
          d="M84 65C66 82 58 112 59 146"
          fill="none"
          opacity="0.2"
          stroke="#f4efff"
          strokeLinecap="round"
          strokeWidth="7"
        />
        <path
          d="M166 73C181 101 182 143 171 177"
          fill="none"
          opacity="0.13"
          stroke="#2d1c67"
          strokeLinecap="round"
          strokeWidth="5"
        />
      </g>

      <g className="seed-core" filter={`url(#${coreGlow})`}>
        <path d="M120 72L133 88L120 105L107 88Z" fill={`url(#${coreGradient})`} />
        <path d="M120 76L127 88L120 100L113 88Z" fill="#eafff8" opacity="0.55" />
      </g>

      <g className="seed-face">
        <g className="seed-eyes">
          <g className="seed-eye seed-eye-left">
            <ellipse cx="95" cy="120" fill={`url(#${eyeGradient})`} rx="10" ry="14" />
            <ellipse cx="92" cy="115" fill="#fff" opacity="0.94" rx="3.2" ry="4.2" />
            <circle cx="98" cy="124" fill="#9cf2d8" opacity="0.42" r="1.4" />
          </g>
          <g className="seed-eye seed-eye-right">
            <ellipse cx="145" cy="120" fill={`url(#${eyeGradient})`} rx="10" ry="14" />
            <ellipse cx="142" cy="115" fill="#fff" opacity="0.94" rx="3.2" ry="4.2" />
            <circle cx="148" cy="124" fill="#9cf2d8" opacity="0.42" r="1.4" />
          </g>
        </g>
        <path d="M112 141C116 145 124 145 128 141" fill="none" opacity="0.78" stroke="#2a2058" strokeLinecap="round" strokeWidth="2.4" />
      </g>

      <g className="seed-petals" stroke="#d8caff" strokeOpacity="0.2" strokeWidth="1">
        <path className="seed-side-plate" d="M51 128C70 142 94 174 113 219C91 214 70 198 58 176C50 161 47 142 51 128Z" fill={`url(#${leafGradient})`} />
        <path className="seed-side-plate" d="M189 128C170 142 146 174 127 219C149 214 170 198 182 176C190 161 193 142 189 128Z" fill={`url(#${leafGradient})`} />
        <path className="seed-inner-plate" d="M73 174C93 181 108 198 120 219C99 217 81 199 73 174Z" fill={`url(#${petalGradient})`} />
        <path className="seed-inner-plate" d="M167 174C147 181 132 198 120 219C141 217 159 199 167 174Z" fill={`url(#${petalGradient})`} />
        <path d="M58 143C75 157 92 181 106 207" fill="none" opacity="0.34" stroke="#eee7ff" strokeLinecap="round" strokeWidth="2" />
        <path d="M182 143C165 157 148 181 134 207" fill="none" opacity="0.28" stroke="#eee7ff" strokeLinecap="round" strokeWidth="2" />
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
