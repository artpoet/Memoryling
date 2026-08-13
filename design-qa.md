# Memoryling procedural seed visual QA

- Source visual truth: `C:\Users\user\AppData\Local\Temp\memoryling-approved-seed-crop.png`
- User annotation for edge thickness: `C:\Users\user\AppData\Local\Temp\codex-clipboard-bbc3e051-1a81-4bdc-a167-fb041e31c39d.png`
- Implementation capture: `C:\Users\user\AppData\Local\Temp\memoryling-procedural-v8-soft-edge-shadow.png`
- Full comparison: `C:\Users\user\AppData\Local\Temp\memoryling-v8-soft-shadow-full-comparison.png`
- Focused annotated comparison: `C:\Users\user\AppData\Local\Temp\memoryling-v8-soft-edge-focus.png`
- State: seed-stage pet, neutral resting frame, no completion mark
- Viewport: implementation rendered headlessly at 320 × 320 CSS px and device scale 1
- Dimensions: source concept crop 190 × 230 px; implementation 320 × 320 px
- Normalization: both creatures are shown at equal display height with aspect ratio preserved

## Findings

- No actionable P0／P1／P2 mismatch remains for the clarified meaning of plate thickness. Renderer v7 keeps the clean lower seam while making the pale-lilac plate surface—not the dark backing—the visible protruding layer.
- Each code-generated pale-lilac side plate now crosses the main shell silhouette by roughly 6–7 viewBox units through the outside midsection, and its highlight follows that new outer curve. A dark-violet rim extends only another roughly 2–3 units behind it, so the focused comparison reads as a raised bright plate with a thin depth edge.
- V6's predominantly dark exposed rim was a P2 interpretation error because the bright surface still appeared trapped inside the body. V7 corrects the layer order and contour rather than merely changing the rim color.
- V7's narrow shadow still ended with a visibly hard outer contour. V8 adds a separate 2.8-unit Gaussian-blurred shadow behind the thin edge, creating a soft violet falloff while leaving the pale-lilac plate and its highlight sharp.
- The accepted v5 shell width, pointed apex, rounded base, eye size／spacing, memory core, and restrained facets remain unchanged.
- The procedural renderer remains cleaner and less irregular than the AI reference. This is an accepted P3 difference: production must remain deterministic, layer-addressable program-generated SVG rather than a raster asset.

## Required fidelity surfaces

- Fonts and typography: not applicable to the creature-only comparison; pet status typography is unchanged.
- Spacing and layout rhythm: the full silhouette and face retain v5 proportions; only the bright side-plate outer contour expands locally by roughly 6–7 viewBox units, followed by a roughly 2–3-unit shadow edge. Both layers retract before the rounded base and remain inside the unchanged 320 × 320 compact envelope.
- Colors and tokens: violet body, indigo eyes, mint core, pale-lilac plates, restrained glow, and low-opacity lavender facet planes remain consistent with the accepted concept family. The protruding region continues the pale-lilac plate gradient and highlight; the backing edge is translucent dark violet rather than black, and its darker blurred duplicate fades outward rather than ending abruptly.
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
15. User review identified that V6 still exposed mostly a dark rim, so the protrusion did not visibly continue the bright plate surface. A red-light contract first required a V7 renderer and exact wider bright-side-plate paths; it failed against V6 before implementation.
16. V7 expands the pale-lilac surfaces beyond the body, moves their highlights onto the new outer curves, and reduces the dark backing to a thin edge. The final full and focused comparisons show bright material across the marked protruding regions with no remaining actionable P0／P1／P2 issue.
17. User review asked for the remaining shadow edge to be softly feathered. A red-light contract required a V8 renderer, a dedicated filter ending in `-plate-shadow`, two shadow contours, and a `2.8` Gaussian blur before implementation.
18. V8 keeps the accepted bright protrusion and exact geometry, reduces the solid rim opacity, and places the blurred duplicate behind it. Full and enlarged comparisons show a restrained soft falloff with no dirty halo, loss of edge clarity, or new P0／P1／P2 issue.

## Follow-up polish

- P3: future renderer work could add slightly more irregular biological contour variation, without changing the accepted v8 facet contrast, bright-surface protrusion, or restrained shadow softness and without importing a raster pet asset.

final result: passed
