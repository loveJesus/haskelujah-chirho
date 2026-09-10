<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Kind-binder lexical scope — 2026-09-10

L.J. directed continued work while asleep. Baseline `18bba3ad`; owner
HASKELUJAH/gpt_chirho in isolated `gpt-kind-scope-chirho`. Canonical progress
row 481, start `2026-09-10T06:28:04.391Z`. Local/reversible compiler repair;
builder and DB lease claimed in room message 21907. No deployment or public
percentage-label policy change, no unrelated warning sweep.

## Brick 1: scope, placement and acceptance

The preceding lane measured `f Int -> (forall f. f -> f) -> f Int` and its
inner-first counterpart: our current checker rejects E0300; GHC 9.14.1 runs
both and prints `42/7`. The kind checker permanently overwrites the outer
name in its forall arm. Its quantified-constraint arm has the same write.
An independent read-only audit checks the consumers while implementation proceeds.

The 3,700-line `kind_chirho.rs` is oversized. Extract related binder/conversion
logic beneath `kind_chirho/`, keeping focused new files below 1,000 lines and
tests in a separate sibling. No new crate/dependency. This is a reversible
placement decision with low correction cost, not a new kind representation.

There are two contracts, not one interchangeable map: ordinary type-kind
inference looks up the kind of a type variable in `env_chirho`; interpreting
a source type as a kind uses `kind_var_cache_chirho` for kind-variable identity.
Establish each binder's correct state, allocate fresh local identities and
restore only touched entries on exit, preserving genuinely new free variables.
Do not clone a growing environment per binder or widen a permissive fallback.
Kind-variable annotations and quantified constraints need explicit controls.

Acceptance: GHC-confirmed valid shadowing executes exactly; invalid kind
applications still diagnose E0300; repeated annotations share the intended
kind variable without inner binders capturing an outer one. Keep existing
required/invisible binder behavior and report unsupported forms honestly.
No corpus gain predicted before reachability is measured.

## Checklist

- [x] Verify clean source/room, isolated branch, runtime identity and canonical row.
- [x] Reproduce positive and negative controls against the current CLI and GHC.
- [x] Add failing regression tests and audit both kind-conversion contracts.
- [x] Extract and repair bounded lexical scope; update its workflow documentation.
- [x] Focused tests, typing/driver integration, canaries and relevant execution gates.
- [x] Freeze source, explicit fresh CLI, two full passes per upstream axis;
      byte-identical repeated sets, named deltas, stable binary provenance.
- [x] Meaningful landing gates, named-path commits, push/read-back verification,
      main-path smoke checks and canonical DB closure.

## Resume and evidence

Scratch: `/private/tmp/haskelujah-kind-scope-chirho.CXoVPy`.
Prior repro: `/private/tmp/haskelujah-forall-scope-chirho.AI6njz/HigherKindScopeChirho.hs`.
Baseline: workspace 3344 passing; upstream 879/938 accept and 223/767 reject;
curated 537/537 with 514 compared execution oracles. These are prior measurements,
not results for an edited tree. Existing clippy and structural debt remain open.

## Focused results

- Explicit baseline CLI rebuild at `18bba3ad`, SHA256
  `9d449ed5521d4798a5fa626ad08233409d023ecc393f2afc423a5fe363a19c32`.
  HigherKindScope reproduces three E0300s while fresh GHC 9.14.1 prints `42/7`.
- Five new scope controls fail against the old implementation; the genuine
  inconsistent-kind control passes before and after. Full typing after repair:
  335/335, including the seventh annotation/cache-scope control.
- Four identical source programs freshly execute warning-free under GHC 9.14.1
  and match STG/LLVM/Cranelift: ordinary and required higher-kinded shadowing,
  explicit kind-annotation shadowing, and quantified-constraint shadowing.
  The blanket witness-class control enables MonoLocalBinds as GHC recommends
  for its local-inference contract, not a disabled warning flag. The bad-kind
  source is independently rejected by GHC-83865 and by our kind-mismatch diagnostic.
- Driver integrations: typing 23, canaries 7, dictionary evidence 10, given
  equalities 10, rigid variables 14; all pass, no ignored/filtered cases.
- `kind_chirho.rs` loses 377 lines; focused conversion/scope files are below 400/60 lines.
  This is not a claim that the remaining 3,364-line root meets the size gate.
- The remaining 59 accept-axis failures were checked individually against the
  rebuilt CLI; none is newly accepted. This is reachability triage, not a full
  accept-subset regression gate or a published measurement.

## Frozen gate completed

Source checkpoint 3683dae897963c4d61bc3737796a7f2969a76bc8 was pushed and
independently matched by ls-remote before the broad gate. Fresh explicit debug
CLI SHA256 f83c9fc56f51eeaf6107032cfbb0b9aad8d60f2515864977edd47ce3bdb321ab.
All four upstream passes assert HEAD/digest before and after: 879/938 accept,
223/767 reject, repeated lists byte-identical to each other and the previous
artifacts, zero gains/losses, unresolved timeouts or unexpected exits.

The exact unfiltered workspace command with RUST_MIN_STACK=16777216 completed
with observed Cargo exit 0: 3356 passed across 75 targets, zero failed, ignored,
measured or filtered. Driver library 1773, typing 335, typing integration 23,
canaries 7, and curated 537/537 are included. All 514 curated execution-source
hashes still match the previous complete GHC 9.14.1 reference; this lane did not
rerun all 514 reference programs. The raw completed workspace log SHA256 is
3c90864404f5bca70a59ffc9e042e91707e48e4dbedb6662d9157892da1e87dc.

No Rust compiler warnings in that workspace command. Targeted typing/driver
all-target clippy exits zero with 552 warning messages (duplicates included),
none with a primary span in the new kind modules or edited integration file.
Existing lint/size debt remains explicitly open. Independent source audit found
no blocker; full class-kind validation, standalone-signature/head reconciliation,
rigid kind schemes and synonym alpha-renaming are not claimed.

## Landing and closure

Evidence commit 1668907e4e368eaf4b7b481781f903af2c35d8b1 was pushed on the
kind-scope branch, fast-forwarded to main, then pushed and verified against the
exact remote main ref. Fresh main CLI SHA256
6d3943e04a0b6593cdcb297c20c7e2ac4f2710cb7df90e7c4828a084f908ef64 passed
all ten bounded main-path smokes with stable HEAD/digest. The in-place module
discovery path and the four new kind execution programs were exercised; both
negative controls required their actual kind/rigid diagnostics. Smoke record
SHA256 d9c8da827067ffe89429ec6d6f8f1e1d4d2acc113a7c0fbb9729677ea7626406.
Canonical row 481 closes with that evidence; the separate flat-type row 482
remains open and retains the builder/DB lease. No public-label policy decision
or deployment was made while L.J. sleeps.
