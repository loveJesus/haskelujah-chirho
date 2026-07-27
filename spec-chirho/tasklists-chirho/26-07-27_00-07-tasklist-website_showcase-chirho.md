*For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)*

# Website showcase slice — claude2_chirho

L.J. direction #8080/#8081/#8082 (2026-07-27 00:03, room haskelujah-chirho, topic direction-chirho):
rebuild the website as a maxed-out demonstration of design capability — advanced visual techniques,
high-quality 3D, otherworldly animation, exceptional palette "that fits the book", novel fonts —
**always fitting the actual needs of the software** (#8082). ≥3 fine-toothed iteration passes before
"ok" (#8081); ship a `/{websitename}-website-guide-chirho.md` route describing the process for other
agents; work autonomously (say / ntfy.sh/chirho_vive_siempre only if truly blocked).

## Brick-1 decisions (architecture/placement)

- **Home**: rebuild inside existing `site-chirho/` (SvelteKit 2 / Svelte 5 / adapter-cloudflare;
  deploys via `bun run deploy-chirho` → Cloudflare Pages project `haskelujah-site-chirho`,
  live at haskelujah.org). `docs/` (plain-HTML GitHub Pages variant) is legacy — untouched this slice.
- **Concept — THE ILLUMINATED COMPILER**: the Book's own palette (illuminated-manuscript tradition:
  gold leaf, lapis/ultramarine, vermillion, parchment, ink) fused with modern compiler-tool
  credibility. Signature visual: a real-time 3D **rose window with 12 panes = the 12 compiler
  phases**, λ at its heart; light-through-glass animation; dark nave (hero) → lit parchment codex
  (content) scroll transition. Authentic to the project's identity (John 3:16 in every file), not
  decoration.
- **Serves the software's needs**: install-first hero (copyable command), scroll pipeline story that
  TEACHES the 12 phases, honest measurement plaque with BOTH published numbers sourced from the
  committed artifacts (`spec-chirho/ghc-should-compile-measurement-chirho.txt` +
  `ghc-should-fail-measurement-chirho.txt`), README-verified feature claims, Limitations kept.
- **Truth pass is mandatory**: current page still says 88.8% / 833/938, omits should_fail entirely,
  and self-contradicts (Test Runner both "done" feature and roadmap item) — same drift disease as
  FIND 2. Every number on the new site traces to a committed artifact; no hardcoded drift-prone
  claims without a source + date.
- **Deps**: `three` (pinned latest stable) is the one significant new runtime dep. Fonts self-hosted
  (OFL: Cormorant/EB Garamond display, Cinzel monumental caps, JetBrains Mono code) — Google Fonts
  CDN and unpkg asciinema CDN both dropped. Custom lightweight .cast player (~JSONL replay) replaces
  the CDN player for full theming + zero external origins.
- **Layout**: components under `site-chirho/src/lib-chirho/` (alias exists), files <1.5k lines,
  dirs <15 entries; every new file gets the John 3:16 header; all identifiers Chirho-suffixed.
- **Assets**: GPT Image 2 (Runware/OpenAI keys) via subagents — stained-glass textures, illuminated
  drop caps, parchment tile, gold ornaments, og:image; optional Pixverse v6 (Runware) shimmer loop;
  Blender MCP if the connected instance is live, else procedural Three.js tracery.

## Checklist

- [x] Recon: current site read end-to-end; deploy pipeline + domain confirmed; lane unclaimed
- [ ] Tasklist committed; progress DB start row (insert-only, agent claude2_chirho); room claim post
- [ ] Research pass (subagent w/ Perplexity key): 2026 SOTA motion-design + 3D dev-tool sites
- [ ] Truth pass: read both measurement artifacts + README + cli subcommand source; claims table
- [ ] Design system: tokens (palette/type/spacing), fonts bundled, base layout
- [ ] Asset generation (subagents; iterate until exceptional)
- [ ] 3D rose-window hero (Three.js) + reduced-motion/static fallback + mobile fallback
- [ ] Scroll-driven 12-phase pipeline story
- [ ] Content sections rebuilt (tools canon-table, plaque, performance w/ provenance, demo terminal,
      playground honesty, get-started, ecosystem, limitations, roadmap, footer)
- [ ] Guide route: `static/haskelujah-website-guide-chirho.md` → `/haskelujah-website-guide-chirho.md`
- [ ] Iteration pass 1 — fine-toothed comb (Playwright: 390/768/1440px, console, keyboard, contrast,
      reduced-motion; design/UX problem list → fixes)
- [ ] Iteration pass 2 — same comb, deeper beautify/complexify
- [ ] Iteration pass 3 — same comb; only then "ok"
- [ ] Gates: svelte-check + build zero errors/warnings; zero console errors; a11y clean; lockfile
      committed
- [ ] Land: commit site-chirho/** + my spec files ONLY; deploy; room announcement + AICEO milestone
      report; workflow DAG under spec-chirho/workflows-chirho/; DB row closed

## Shared-tree discipline

claude_chirho's validity slice owns crates/** and the cargo builder — this lane runs ZERO cargo and
touches ZERO crate files. RecursiveDo slice stays mine, parked until after the site ships (or
re-sequenced if L.J. redirects). Progress DB stays uncommitted (their sweep); spec files go in via
`git add -f` (gitignore FIND 3 unresolved, L.J.'s call).
