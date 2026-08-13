# Memoryling procedural seed visual QA

- Source visual truth: `C:\Users\user\AppData\Local\Temp\memoryling-approved-seed-crop.png`
- User annotation for edge thickness: `C:\Users\user\AppData\Local\Temp\codex-clipboard-bbc3e051-1a81-4bdc-a167-fb041e31c39d.png`
- Implementation capture: `C:\Users\user\AppData\Local\Temp\memoryling-procedural-v6-edge-rims-pass2.png`
- Full comparison: `C:\Users\user\AppData\Local\Temp\memoryling-ai-v5-v6-outer-rims-full.png`
- Focused annotated comparison: `C:\Users\user\AppData\Local\Temp\memoryling-marked-v5-v6-outer-rims-focus.png`
- State: seed-stage pet, neutral resting frame, no completion mark
- Viewport: implementation rendered headlessly at 320 × 320 CSS px and device scale 1
- Dimensions: source concept crop 190 × 230 px; implementation 320 × 320 px
- Normalization: both creatures are shown at equal display height with aspect ratio preserved

## Findings

- No actionable P0／P1／P2 mismatch remains for the corrected meaning of plate thickness. Renderer v6 removes the mistakenly centered thickness block and restores a clean balanced lower seam.
- Two code-generated dark-violet rim shapes now sit behind the left／right side plates and extend beyond the main shell silhouette through the outside midsection. The focused comparison aligns directly with the user's orange-marked regions.
- Pass 1 allowed the rims to extend too far around the bottom and read like extra wings. Pass 2 shortens and lightens them so they taper away before the base, reading as plate thickness rather than a new appendage or full outline.
- The accepted v5 shell width, pointed apex, rounded base, eye size／spacing, memory core, and restrained facets remain unchanged.
- The procedural renderer remains cleaner and less irregular than the AI reference. This is an accepted P3 difference: production must remain deterministic, layer-addressable program-generated SVG rather than a raster asset.

## Required fidelity surfaces

- Fonts and typography: not applicable to the creature-only comparison; pet status typography is unchanged.
- Spacing and layout rhythm: the full silhouette and face retain v5 proportions; only the side-plate outer contour expands locally by roughly 5–8 viewBox units. The rim retracts before the rounded base and remains inside the unchanged 320 × 320 compact envelope.
- Colors and tokens: violet body, indigo eyes, mint core, pale-lilac plates, restrained glow, and low-opacity lavender facet planes remain consistent with the accepted concept family. The exposed side rims are translucent dark violet rather than black.
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
11. User asked to continue the crystal／biological polish and identified that the two lowest plates should be slightly staggered with visible thickness. A red-light contract required four facet layers, explicit back／front plate order, asymmetric paths, and one thickness edge before implementation.
12. V5 pass 1 established the overlap but made the rear plate too dark, reading as a hole. Pass 2 restored the shared petal gradient and narrowed／lightened the thickness edge. The final full and focused comparisons show a subtle right-back／left-front overlap with no remaining P0／P1／P2 issue.
13. User clarified with an annotated screenshot that “thickness” meant the left／right plate edges should protrude beyond the body outline, not that the two inner plates should overlap at the center. A red-light contract required two outer rims and no center thickness layer before implementation.
14. V6 pass 1 moved thickness to the correct sides but extended too far toward the bottom, resembling extra wings. Pass 2 shortened and softened the rims. The final full and annotated focused comparisons show the protrusion at the requested side edges with a clean lower seam and no remaining P0／P1／P2 issue.

## Follow-up polish

- P3: future renderer work could add slightly more irregular biological contour variation, without changing the accepted v6 facet contrast or side-rim protrusion and without importing a raster pet asset.

final result: passed
