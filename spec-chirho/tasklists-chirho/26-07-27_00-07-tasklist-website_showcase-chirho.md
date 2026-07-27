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
- [x] Tasklist committed (431b3a6); progress DB start row; room claim post (#8095)
- [x] Research pass — subagent brief delivered + claude_chirho's handoff (#8117) integrated
- [x] Truth pass: both artifacts parsed AT BUILD TIME (compat-chirho.ts, fails on drift); 12 CLI
      subcommands verified from cli source; stats from stats-chirho.sh dated; benchmark table
      KILLED (no in-repo timing artifact); no primes sieve anywhere (claude_chirho's miscompile)
- [x] Design system: manuscript tokens (WCAG-measured), Cormorant/EB Garamond/Cinzel/Recursive
      self-hosted OFL, CDNs dropped
- [x] Asset approach: og from the real rendered hero; Runware/OpenAI recipes verified + parked in
      scratchpad (asset-api-recipes-chirho.md) for future art passes — procedural art shipped v1
- [x] 3D rose-window hero: 12 shader panes (clockwise, phase-hued), λ oculus, god-ray fakes, dust,
      kindle-on-load sequence, scroll dolly THROUGH the oculus; DPR clamp, IO+visibility pause,
      context-loss CSS fallback, adaptive degrade ladder, reduced-motion = static frame
- [x] 12-phase pipeline story: sticky mini rose window lights panes per stage (IO-driven),
      hand-accurate illustrative IR dumps, labeled as such
- [x] Sections rebuilt: plaque+manifesto, canon table, scriptorium (real .cast replay via zero-dep
      player; FAKE playground deleted), begin, confessio, roadmap, colophon; ecosystem folded into
      canon/begin; performance section dropped until a real timing artifact exists
- [x] Guide route live: /haskelujah-website-guide-chirho.md
- [x] Iteration pass 1 — 6 findings fixed (window scale ×2, missing parchment bg, stale playground
      copy, program/output mismatch, descend-hint collision, Timer deprecation)
- [x] Iteration pass 2 — kindle sequence added, wax seals, oculus glow cap, clockwise order match,
      mobile h-overflow fixed (grid minmax), reduced-motion + keyboard + tooltip verified
- [x] Iteration pass 3 — prod-build sweep: 0 console errors, ~172KB gz total JS incl. lazy three
      chunk, fresh og from the real hero, final screenshots 390/1200/1440
- [x] Gates: svelte-check --fail-on-warnings 0/0; production build clean; lockfile committed
- [ ] Land: deploy via wrangler; live-curl verify; room announcement + AICEO report; DB row closed

~~Note for the artifact owner: should_compile header date looked like a typo (07-07 vs an
assumed 07-26 commit).~~ **CORRECTION (2026-07-27 ~02:00): my note was the wrong one.** Verified
by git show: dbf261e2 = 2026-07-06 00:56, artifact commit b58000c9 = 2026-07-06 23:23 — so a
"measured 2026-07-07" header (just past midnight) is plausible, not a typo. I published an
unverified date inference in a truth doc — the exact failure class this slice corrects.
claude_chirho supersedes that artifact with a freshly dated re-measurement rather than patching
an unverifiable field (#8378), which is the cleaner resolution.

## Post-landing addendum (2026-07-27 ~01:55)

- [x] PRECISION FIX deployed after claude_chirho's nondeterminism find (#8361: type checker
      flips accept/reject per process on T25266.hs, ±1 file observed on the corpus): the plaque
      no longer renders the artifacts' third-significant-figure percentages — display is
      whole-number "≈ 90% / ≈ 18%" computed from the committed counts, plus a method note
      naming the single-sweep method, the observed ±1 variance, and the open nondeterminism.
      Fractions still render today's COMMITTED artifact values (850/938, 141/767) per their
      explicit instruction; the 149 should_fail artifact is deliberately uncommitted upstream
      and lands on their SLICE ARTIFACT COMMITTED signal → one rebuild+redeploy here.
      The sieve miscompile is NOT mentioned on the site (no conflation).
- [ ] PENDING on claude_chirho's ARTIFACTS SUPERSEDED signal (#8647: both corpora re-measuring
      on the deterministic binary post-47723b8c; should_compile already 849/938 point-value +
      deterministic, should_fail in flight): when the superseded artifacts COMMIT —
      (a) Confessio (1) flips from "still reproduces" to fixed-and-verified language (or is
      retired from the ledger with the fix noted in the method line — decide at edit time);
      (b) method note drops the "±1 observed" sentence, keeps floored-percentage policy with
      simplified rationale; (c) QUOTE-AS point value + DETERMINISTIC stability line flow
      through the parser automatically, zero edits; then one clean-worktree rebuild+redeploy.
- [x] SLICE ARTIFACT COMMITTED cycle (ad6a6dde → their #8392): parser contract updated for the
      new header shape (# measured date + # code measured hash), # STABILITY parsed and rendered
      VERBATIM per card (differing axis confidence not flattened), QUOTE-AS now live
      ("~90% (849-850 of 938)" / "~19% (149 of 767)"), Confessio gains the two newly-published
      README limitations worded symptom-only (their #8395 mechanism retraction confirms the
      caution: no mechanism claims on the site). Progress DB swept in per their housekeeping ask.

## Shared-tree discipline

claude_chirho's validity slice owns crates/** and the cargo builder — this lane runs ZERO cargo and
touches ZERO crate files. RecursiveDo slice stays mine, parked until after the site ships (or
re-sequenced if L.J. redirects). Progress DB stays uncommitted (their sweep); spec files go in via
`git add -f` (gitignore FIND 3 unresolved, L.J.'s call).
