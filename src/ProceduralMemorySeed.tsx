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
      data-renderer="procedural-svg-v6"
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

      <ellipse cx="120" cy="229" fill="#4e388d" opacity="0.12" rx="72" ry="11" />

      <g className="seed-sprouts" fill={`url(#${leafGradient})`}>
        <path d="M108 56C91 52 77 36 79 13C98 16 115 35 108 56Z" />
        <path d="M132 54C130 34 144 15 162 11C165 34 151 51 132 54Z" />
        <path d="M101 48C96 34 89 23 82 16" fill="none" opacity="0.34" stroke="#eee7ff" strokeLinecap="round" strokeWidth="2" />
        <path d="M139 46C148 34 155 22 160 15" fill="none" opacity="0.3" stroke="#eee7ff" strokeLinecap="round" strokeWidth="2" />
      </g>

      <g filter={`url(#${bodyGlow})`}>
        <path
          className="seed-shell"
          d="M120 37C108 49 85 52 68 69C50 87 42 116 43 150C44 188 69 215 102 226C113 230 127 230 138 226C171 215 196 188 197 150C198 116 190 87 172 69C155 52 132 49 120 37Z"
          fill={`url(#${bodyGradient})`}
        />
        <path
          d="M80 65C61 82 52 112 53 146"
          fill="none"
          opacity="0.2"
          stroke="#f4efff"
          strokeLinecap="round"
          strokeWidth="7"
        />
        <path
          d="M170 73C186 101 188 143 177 177"
          fill="none"
          opacity="0.13"
          stroke="#2d1c67"
          strokeLinecap="round"
          strokeWidth="5"
        />
        <g className="seed-shell-facets" pointerEvents="none">
          <path className="seed-shell-facet" d="M120 39C108 50 89 55 72 69L95 76L120 58Z" fill="#d7c6ff" opacity="0.12" />
          <path className="seed-shell-facet" d="M120 39L120 58L145 77L169 69C153 53 132 49 120 39Z" fill="#5f43aa" opacity="0.1" />
          <path className="seed-shell-facet" d="M47 143C53 173 72 203 102 223L92 184L66 149Z" fill="#d2c0ff" opacity="0.07" />
          <path className="seed-shell-facet" d="M193 143C187 174 168 204 138 223L149 183L175 148Z" fill="#3a2677" opacity="0.07" />
        </g>
      </g>

      <g className="seed-core" filter={`url(#${coreGlow})`}>
        <path d="M120 72L133 88L120 105L107 88Z" fill={`url(#${coreGradient})`} />
        <path d="M120 76L127 88L120 100L113 88Z" fill="#eafff8" opacity="0.55" />
      </g>

      <g className="seed-face">
        <g className="seed-eyes">
          <g className="seed-eye seed-eye-left">
            <ellipse cx="92" cy="120" fill={`url(#${eyeGradient})`} rx="10" ry="14" />
            <ellipse cx="89" cy="115" fill="#fff" opacity="0.94" rx="3.2" ry="4.2" />
            <circle cx="95" cy="124" fill="#9cf2d8" opacity="0.42" r="1.4" />
          </g>
          <g className="seed-eye seed-eye-right">
            <ellipse cx="148" cy="120" fill={`url(#${eyeGradient})`} rx="10" ry="14" />
            <ellipse cx="145" cy="115" fill="#fff" opacity="0.94" rx="3.2" ry="4.2" />
            <circle cx="151" cy="124" fill="#9cf2d8" opacity="0.42" r="1.4" />
          </g>
        </g>
        <path d="M112 141C116 145 124 145 128 141" fill="none" opacity="0.78" stroke="#2a2058" strokeLinecap="round" strokeWidth="2.4" />
      </g>

      <g className="seed-petals" stroke="#d8caff" strokeOpacity="0.2" strokeWidth="1">
        <path className="seed-side-plate-rim" d="M45 126C37 142 37 162 46 180C55 197 70 209 88 215L91 209C74 203 61 191 52 175C44 158 44 142 50 131Z" fill="#3f2c77" opacity="0.6" stroke="none" />
        <path className="seed-side-plate-rim" d="M195 126C203 142 203 162 194 180C185 197 170 209 152 215L149 209C166 203 179 191 188 175C196 158 196 142 190 131Z" fill="#3f2c77" opacity="0.6" stroke="none" />
        <path className="seed-side-plate" d="M45 128C66 142 93 174 113 219C90 214 66 198 52 176C43 161 41 142 45 128Z" fill={`url(#${leafGradient})`} />
        <path className="seed-side-plate" d="M195 128C174 142 147 174 127 219C150 214 174 198 188 176C197 161 199 142 195 128Z" fill={`url(#${leafGradient})`} />
        <path className="seed-inner-plate" d="M68 174C91 181 108 198 120 219C97 217 77 199 68 174Z" fill={`url(#${petalGradient})`} />
        <path className="seed-inner-plate" d="M172 174C149 181 132 198 120 219C143 217 163 199 172 174Z" fill={`url(#${petalGradient})`} />
        <path d="M78 179C96 187 109 201 118 215" fill="none" opacity="0.2" stroke="#f1eaff" strokeLinecap="round" strokeWidth="1.5" />
        <path d="M163 183C148 190 136 203 126 217" fill="none" opacity="0.17" stroke="#d8caff" strokeLinecap="round" strokeWidth="1.4" />
        <path d="M52 143C71 157 91 181 106 207" fill="none" opacity="0.34" stroke="#eee7ff" strokeLinecap="round" strokeWidth="2" />
        <path d="M188 143C169 157 149 181 134 207" fill="none" opacity="0.28" stroke="#eee7ff" strokeLinecap="round" strokeWidth="2" />
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
