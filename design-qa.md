# Memoryling procedural seed visual QA

- Source visual truth: `C:\Users\user\AppData\Local\Temp\memoryling-approved-seed-crop.png`
- Implementation capture: `C:\Users\user\AppData\Local\Temp\memoryling-teardrop-round-bottom-final.png`
- Full comparison: `C:\Users\user\AppData\Local\Temp\memoryling-ai-vs-programmatic-v3-shell.png`
- Focused silhouette comparison: the full comparison presents the source concept and renderer at equal display height; a second crop is not needed
- State: seed-stage pet, neutral resting frame, no completion mark
- Viewport: implementation rendered headlessly at 320 × 320 CSS px and device scale 1
- Dimensions: source concept crop 190 × 230 px; implementation 320 × 320 px
- Normalization: both creatures are shown at equal display height with aspect ratio preserved

## Findings

- No actionable P0／P1／P2 mismatch remains for the requested silhouette correction. Renderer v3 replaces the rounded egg apex with the concept's visible upper point and closes the lower shell through a broad multi-control-point arc rather than a single center point.
- The plates still wrap the cheeks and meet near the center, but now stop above the outer shell's lowest contour. This prevents the plate seam from visually turning the whole body into a pointed-bottom egg／inverted teardrop.
- The previously corrected eye size, spacing, vertical synchronization, memory-core prominence, and outward sprouts are intentionally unchanged in this silhouette pass.
- The procedural renderer remains smoother and less faceted than the AI reference. This is an accepted P3 difference: the production creature must remain deterministic, layer-addressable program-generated SVG rather than a raster asset.

## Required fidelity surfaces

- Fonts and typography: not applicable to the creature-only comparison; pet status typography is unchanged.
- Spacing and layout rhythm: the face occupies the same upper-middle region; the pointed apex sits between the sprouts, while the plate junction remains visibly above the rounded shell base. The 320 × 320 compact envelope remains unchanged.
- Colors and tokens: violet body, indigo eyes, mint core, pale-lilac plates, and restrained glow remain consistent with the accepted concept family.
- Image quality and asset fidelity: the implementation is vector-sharp and transparent. No generated raster, embedded `<image>`, or runtime bitmap is used; the accepted AI image is reference-only by explicit product decision.
- Copy and content: no creature copy changed; the required access-off text remains outside the renderer.
- Accessibility and behavior: paired blink is one motion unit and the existing reduced-motion path remains intact; the pet's accessible label and interaction surface are unchanged.

## Comparison history

1. Pass 1 found P1 facial maturity and P1 detached lower petals: eyes were too widely spaced, the mouth was too low, and the petals read as a necklace.
2. Pass 1 fix moved the eyes inward, raised the mouth, lowered the memory core, and introduced cheek-side plates.
3. Pass 2 review found the eyes had moved slightly too close and the side plates still did not reproduce the concept's continuous wrapped silhouette.
4. Pass 2 fix settled the eye centers at x=96／144 with larger ellipses, enlarged the memory core, widened the sprouts, and replaced the lower cluster with four continuous side／inner plates that meet at the base.
5. Post-fix full and focused comparisons show no remaining P0／P1／P2 proportion mismatch.
6. User review correctly identified that pass 2 still felt worse because inward movement and eye enlargement compounded each other. The corrected pass restored the original eye size and reduced inward movement to one unit per side; a three-way background render confirms the central space is restored while the improved plates and facial height remain.
7. User review then identified the larger structural drift: the concept is pointed／teardrop-like on top with a rounded lower arc, while renderer v2 still read as an egg. A red-light path assertion failed against the old shell before implementation.
8. Renderer v3 introduced the pointed apex and wider lower shell. The first review found the inner plates still dragged the silhouette into a center point, so their junction was raised above the outer-shell base. The final equal-height comparison confirms that the shell now owns a continuous rounded bottom contour.

## Follow-up polish

- P3: future renderer work could add deterministic faceted highlight planes and slightly more irregular biological edges, without changing the accepted v3 shell geometry or importing a raster pet asset.

final result: passed
