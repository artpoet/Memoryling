# Creature visual baseline

> Status: Accepted direction and implemented seed-stage baseline. Later stages remain reference-only until their deterministic renderer contract is accepted.

## Canonical seed form

Memoryling starts as a compact violet **memory seed**: a pointed upper teardrop silhouette with two short leaf sprouts and a broad rounded lower arc, living indigo eyes, a mint memory-seed motif, and layered petal plating around the lower body. Its working renderer IDs are:

- `stage: seed`
- `bodyModule: memory-seed-egg-v1`
- `palette: violet-mint`
- `motion: calm`

The `memory-seed-egg-v1` body-module value is retained as a legacy-compatible state identifier; it no longer describes the accepted outer silhouette. The desktop body is generated at runtime by the layered React SVG renderer in `src/ProceduralMemorySeed.tsx` (`procedural-svg-v8`). Its shell, translucent facet planes, sprouts, eyes, memory core, lower plates, outer plate rims, soft edge shadows, highlights, and motion remain individually addressable code layers; the desktop pet does not load a raster creature image. The v8 silhouette and face proportions are grounded in the accepted AI concept and the user's annotated edge-thickness reference, but every visible production layer remains deterministic program-generated SVG.

The canonical shell is not a symmetric egg. Its center rises to a visible point between the sprouts, the upper shoulders flow downward like a teardrop, and the lower body widens before closing in a soft rounded arc. Renderer v8 retains the 154-unit shell width and four low-opacity planes that imply organic crystal refraction without turning the creature into a hard gemstone. The paired eyes are a single synchronized motion unit: their primary ellipses share one vertical center, sit at `x=92／148` with the original `rx=10／ry=14` size in the 240-unit viewBox, and blink together. Plate thickness belongs on the left／right outside edges, not at the inner center seam: each pale-lilac side-plate surface itself crosses roughly 6–7 viewBox units beyond the dark main-body silhouette, and its highlight continues along that protruding outer curve. A separate dark-violet rim may extend only another roughly 2–3 units behind the bright plate to explain the step in depth; an independently rendered low-opacity shadow repeats that outside-edge contour with a 2.8-unit Gaussian blur so the darkness falls off softly rather than ending as a hard stripe. All three layers taper away before the rounded base. The lower inner plates remain balanced and meet cleanly above that base. Do not blur the bright plate surface, make the exposed protrusion primarily dark, move thickness back to the center, extend the rims into a second pair of wings around the bottom, turn them into a full dark outline, narrow the body into a tall capsule, raise facet contrast into a hard crown, round off the shell apex into an egg, pull the plate seam into a body-defining point, reintroduce independent eye timing, or detach the lower petals. On the compact pet surface, the required memory-access-off truth remains visible as quiet inline text rather than a pill, button, card, shadow, or other interactive-looking control.

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
