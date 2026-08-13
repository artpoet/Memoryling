# Memoryling procedural seed visual QA

- Source visual truth: `C:\Users\user\AppData\Local\Temp\memoryling-approved-seed-crop.png`
- Implementation capture: `C:\Users\user\AppData\Local\Temp\memoryling-procedural-v2-eye-corrected.png`
- Full comparison: `C:\Users\user\AppData\Local\Temp\memoryling-eye-spacing-three-way.png`
- Focused face comparison: the three-way comparison isolates the eye-size／spacing correction at sufficient resolution; a second crop was not needed
- State: seed-stage pet, neutral resting frame, no completion mark
- Viewport: implementation rendered headlessly at 320 × 320 CSS px and device scale 1
- Dimensions: source 190 × 230 px; source creature crop 104 × 150 px; implementation 320 × 320 px with a 206 × 289 px alpha crop
- Normalization: both creature crops were aspect-fit into equal 250 × 280 comparison regions; the face comparison uses equal display regions while preserving each crop's aspect ratio

## Findings

- No actionable P0／P1／P2 mismatch remains for the requested cute facial proportions. The paired eyes now retain the original `rx=10／ry=14` size and move inward by only one viewBox unit per eye (`94／146` → `95／145`), restoring central breathing room while keeping the face slightly more cohesive. Their vertical centers and blink remain synchronized.
- The eye-to-mouth distance, memory-core prominence, outward sprouts, and side plates now follow the accepted reference's composition. Four continuous program-generated plates replace the detached necklace-like lower petals.
- The procedural renderer remains smoother and less faceted than the AI reference. This is an accepted P3 difference: the production creature must remain deterministic, layer-addressable program-generated SVG rather than a raster asset.

## Required fidelity surfaces

- Fonts and typography: not applicable to the creature-only comparison; pet status typography is unchanged.
- Spacing and layout rhythm: the face occupies the same upper-middle region and the plates meet at the base without clipping. The 320 × 320 compact envelope remains unchanged.
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

## Follow-up polish

- P3: future renderer work could add deterministic faceted highlight planes to the shell and plates, without changing the accepted face geometry or importing a raster pet asset.

final result: passed
