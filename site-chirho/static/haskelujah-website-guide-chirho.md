<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# How the Haskelujah website was made — a guide for other agents

This page (haskelujah.org) was rebuilt 2026-07-27 by claude2_chirho on L.J.'s direction:
"maxed out… advanced visual techniques, high-quality 3D, otherworldly animations, exceptional
color palettes that fit the book, novel fonts — always fitting the actual needs of the software."
This file is the repeatable method, written so another agent can do the same for another project.

## 1. Concept before pixels

Find the ONE metaphor that is both true to the project's identity and encodes the product's
actual model. Here: **The Illuminated Compiler** — the project is Haskell + Hallelujah with
John 3:16 in every source file, so the visual language of illuminated manuscripts (gold leaf,
lapis, vermilion, parchment) is authentic, not costume. The signature visual must TEACH:
our rose window has twelve panes because the compiler has twelve phases, with λ at the center.
A beautiful thing that encodes nothing is a screensaver; reject it.

## 2. Serve the software's needs first (the non-negotiable)

A compiler site must: (a) show the install command within seconds, (b) teach the architecture,
(c) prove compatibility honestly, (d) link real docs. Design decorates THESE; it never replaces
them. The research cliché list is real: no fake typing-terminal heroes, no fake playgrounds,
no benchmark charts without methodology. If interactivity can't be real yet, label the sketch
and ship the real path (`cargo install … && … repl`) beside it.

## 3. Truth is a build step, not a copywriting habit

Every number on the page is parsed AT BUILD TIME from committed measurement artifacts
(`src/lib-chirho/data-chirho/compat-chirho.ts` imports `spec-chirho/*.txt?raw` and fails the
build if the format drifts). Publish BOTH axes — what we accept correctly AND what we reject
correctly — with date + commit + link to the artifact. Claims that can't be traced to the repo
(stale test counts, unverifiable package tallies, benchmark timings with no artifact) get cut,
not softened. The old page drifted three different compatibility numbers; a build-time parser
makes that structurally impossible.

## 4. The craft decisions (steal these)

- **Palette law** (WCAG-measured): gold speaks in the dark, ink speaks on parchment, lapis
  speaks in both. Gold text on parchment fails contrast — gold is ornament-only on light.
  Pass/fail semantics use period pigments: verdigris / vermilion.
- **Type**: Cormorant (display serif), EB Garamond (body), Cinzel (Roman caps for numerals and
  labels), Recursive (code — its variable MONO/CASL axes make one file span prose to strict
  monospace). All OFL via @fontsource-variable, self-hosted, zero CDN.
- **Dark/light rhythm**: nave (dark) → codex (parchment) → back — the page alternates like the
  liturgy of hours; each register gets its own contrast-checked tokens.
- **Grain, not JPEGs**: parchment is a flat warm field + one fixed SVG feTurbulence overlay at
  ~3% soft-light. Photographic parchment textures read as a tavern menu.
- **Ornament budget**: one drop cap, one fleuron rule per section head, one arch motif. Stop.

## 5. 3D that respects the visitor

Three.js WebGL2, procedural (no model downloads): 12 shader panes (polar leaded-glass cells,
noise-warped), λ oculus via SDF capsules, additive fake god-rays with dither, unsorted additive
dust. Guardrails, all mandatory: DPR clamped (1.5 desktop / 1.0 mobile), three.js dynamically
imported after LCP, IntersectionObserver + visibilitychange pause the loop, webglcontextlost
swaps to a CSS fallback (conic-gradient window — no asset), prefers-reduced-motion renders ONE
static frame, and an adaptive ladder drops DPR → particles under sustained slow frames.
The one "chapter moment": scroll dollies the camera through the oculus into the parchment codex.
Motion elsewhere is CSS, gated behind prefers-reduced-motion.

## 6. Multi-agent etiquette

Claim the lane in the room before touching shared trees; cede cleanly on overlap. Copy no
numbers from the old site (drift is contagious); re-verify everything at the source. Zero
builds on a toolchain another agent owns — this entire site was built without one cargo
invocation. Surface repo-hygiene contradictions to the human rather than "fixing" them silently.

## 7. The iteration ritual (L.J.'s rule: three passes minimum)

After the site works, comb it three separate times before calling it done — Playwright
screenshots at 390/768/1440, console must be empty, keyboard-only walk, contrast spot-checks,
reduced-motion run, copy read aloud for voice. Each pass files problems THEN fixes them;
opportunities to beautify or deepen count as findings too. Gates before "ok": svelte-check
clean with --fail-on-warnings, production build clean, lockfile committed.

*Soli Deo gloria.*
