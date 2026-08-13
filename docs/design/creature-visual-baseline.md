# Creature visual baseline

> Status: Accepted direction and implemented seed-stage baseline. Later stages remain reference-only until their deterministic renderer contract is accepted.

## Canonical seed form

Memoryling starts as a compact violet **memory seed**: an egg-like silhouette with two short leaf sprouts, living indigo eyes, a mint memory-seed motif, and layered petal plating around the lower body. Its working renderer IDs are:

- `stage: seed`
- `bodyModule: memory-seed-egg-v1`
- `palette: violet-mint`
- `motion: calm`

The desktop body is generated at runtime by the layered React SVG renderer in `src/ProceduralMemorySeed.tsx` (`procedural-svg-v1`). Its shell, sprouts, eyes, memory core, lower plates, highlights, and motion remain individually addressable code layers; the desktop pet does not load a raster creature image.

The paired eyes are a single synchronized motion unit: their primary ellipses share one vertical center and they blink together. Do not reintroduce independent eye timing, per-eye transforms, or asymmetrical offsets that can make the face read as misregistered. On the compact pet surface, the required memory-access-off truth remains visible as quiet inline text rather than a pill, button, card, shadow, or other interactive-looking control.

The checked-in transparent PNGs under `src-tauri/icons/` and `src/assets/memoryling-icon.png` are packaging／brand references only. They are not runtime pet sprites, renderer inputs, or future growth states. Earlier AI concept images likewise remain visual references only.

This is the default visible body whenever no later accepted `MorphologyRecipe` exists. Do not replace it with the former CSS-built square-round body, long upright ears, blush cheeks, or an unrelated generated mascot.

## Accepted family direction

The visual direction combines:

- the living eyes and biological presence of the organic concept;
- the restrained sacred-premium finish of the faceted concept;
- adjacent-stage continuity rather than identical anatomy at every stage;
- many bounded variants derived locally from approved evidence, not one fixed five-form roster.

The accepted stage 4 → 5 bridge vocabulary remains reference material:

- short arc petals grow into a covering ring;
- a close narrow leaf develops into layered plates;
- short tail petals extend into a long ribbon.

These references define family grammar and `EvolutionBridge` expectations only. They are not production sprite sheets, fixed professions, or permission to infer form from raw text. The current source still implements only the seed-stage body plus the existing completion mark.

## Implementation guard

`CreatureRenderState` must identify the accepted body with explicit versioned IDs. A generic value such as `baseline` is insufficient because it allows a renderer to drift without changing the contract. Unknown stage or body-module IDs fail closed to this reviewed seed form.

Future stage or variant work must preserve the privacy, lineage, forgetting, and deterministic-module boundaries in ADR-0004. Extend or compose reviewed renderer layers and parameters; do not switch the live pet to a generated raster sprite. Any production visual change must update this file, the renderer ID, tests, and visible verification evidence together.
