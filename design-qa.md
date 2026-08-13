# Memoryling procedural seed visual QA

- Source visual truth: `C:\Users\user\AppData\Local\Temp\memoryling-approved-seed-crop.png`
- Implementation capture: `C:\Users\user\AppData\Local\Temp\memoryling-procedural-v4-wide-pass1.png`
- Full comparison: `C:\Users\user\AppData\Local\Temp\memoryling-ai-v3-v4-width-comparison.png`
- Focused proportion comparison: the three-way board presents the source concept, previous v3, and current v4 at equal display height, making both body width and eye spacing directly visible; a second crop is not needed
- State: seed-stage pet, neutral resting frame, no completion mark
- Viewport: implementation rendered headlessly at 320 × 320 CSS px and device scale 1
- Dimensions: source concept crop 190 × 230 px; implementation 320 × 320 px
- Normalization: both creatures are shown at equal display height with aspect ratio preserved

## Findings

- No actionable P0／P1／P2 mismatch remains for the requested width and eye-spacing correction. Renderer v4 increases shell width from 142 to 154 viewBox units (about 8.5%) while preserving the pointed apex, height, and rounded lower arc.
- Eye centers move symmetrically from `x=95／145` to `x=92／148`, increasing their center distance from 50 to 56 without enlarging the eyes or changing their shared vertical center and synchronized blink. Eye highlights move with their respective eye groups.
- The plates still wrap the cheeks and meet near the center, but now stop above the outer shell's lowest contour. This prevents the plate seam from visually turning the whole body into a pointed-bottom egg／inverted teardrop.
- The shadow, highlights, side plates, and inner plates widen with the shell, preventing the central body from expanding while its supporting layers remain visually narrow.
- The procedural renderer remains smoother and less faceted than the AI reference. This is an accepted P3 difference: the production creature must remain deterministic, layer-addressable program-generated SVG rather than a raster asset.

## Required fidelity surfaces

- Fonts and typography: not applicable to the creature-only comparison; pet status typography is unchanged.
- Spacing and layout rhythm: the body is broader rather than vertically stretched; the face keeps its upper-middle position and gains modest horizontal breathing room. The pointed apex and rounded base remain intact within the unchanged 320 × 320 compact envelope.
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
9. User review identified that v3 remained too narrow overall and asked for slightly wider eye spacing. A red-light contract failed against v3 before implementation. Renderer v4 widened the shell and every dependent contour, then moved each eye three viewBox units outward without changing eye size.
10. The final three-way equal-height comparison shows v4 is visibly fuller than v3 while remaining a vertical seed form; the eye change is noticeable but restrained, and no P0／P1／P2 proportion issue remains for this request.

## Follow-up polish

- P3: future renderer work could add deterministic faceted highlight planes and slightly more irregular biological edges, without changing the accepted v4 shell geometry or importing a raster pet asset.

final result: passed
