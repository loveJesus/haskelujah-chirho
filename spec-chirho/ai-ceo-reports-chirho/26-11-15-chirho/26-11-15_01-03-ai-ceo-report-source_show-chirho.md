<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV) -->

# Numeric Show source checkpoint, unit 762

The next pipeline now checks GHC 9.14.1's actual Show class, defaults, helper declarations and Int/Integer instances, then executes numeric rendering with source append and lazy tails in the Rust interpreter. The isolated source checkpoint is 5b038d980cb35b2f6ef471e6c1c3e9a7c88418e0, feature metadata 828843679fad65e74f9dafc042f48c6569a99287; both are pushed to gh_chirho and read back. Main receives ledger/documentation only. Its compiler crates remain 04116a69eca349f4e7493fa60420d5aa53a4fb52.

This follows the November 14 source-Int account at spec-chirho/ai-ceo-reports-chirho/26-11-14-chirho/26-11-14_19-00-ai-ceo-report-source_int-chirho.md and its attribution correction. Unit 761's source append/resolved fixities were the prerequisite; this report closes 762, without repeating those earlier tests as new execution. GPT is the implementation writer, oracle/runtime executor and sole canonical DB writer. Claude2 provided architecture and bounded source reviews; no independent engineering execution is claimed for it. No contribution or acceptance from an unconsulted lane is invented.

## What changed

The actual source supplies Show/ShowS, class defaults, showList__, shows/showChar/showString/showParen/showSpace, and both numeric instances. Only the smallest declarations touching Integer/Int layouts receive manifest-owned bodies: showSignedInt and Show Integer.showsPrec. Source types and exact original-name ownership are rechecked through HIR, TC and Core. No handwritten Show scheme, source-spelling primitive dispatch or alternate String representation was added.

All three body lambdas enclose ordered WHNF cases: Int p,n; Integer n,p. The primitive receives exactly the evaluated p,n operands, never the tail. Its existing packed String prefix is joined through checked original Base.(++) at Char, so the tail stays lazy. The machine-owned numeric allowance reserves conversion and later expansion work before producing text, with a 64 KiB cap; the primitive holds no heap pointer and cannot enter a nested evaluator. Show joins numeric defaulting by its complete original name and only when every group class has a present context-free Integer instance.

Whole upstream files remain unchanged/hash-pinned, with their existing licence files retained. The product reads embedded source bytes rather than the developer GHC reference tree. Three PATH=/usr/bin:/bin CLI controls execute source/interface/Core linking without GHC available in PATH. This does not prove cargo-install on every platform.

## Executed gates and their limits

- 391 unique scoped unit tests passed: current r5 driver next 204; unchanged RTS 94 / runtime 67 / TC 26 retain their separate 187 library-unit results. No failures or ignored tests. Runtime integration targets and the whole legacy driver suite were not run.
- Workspace all-targets check and dev CLI build passed with 0 diagnostics. Clippy command 0 AND the unchanged changed-span judge 0 found 0 new hits. Its 279 outside warnings remain UNATTRIBUTED; this is not whole-tree lint green or a waiver. No suppressions or judge changes were used.
- GHC 9.14.1: 13 fixture compile checks + 38 queries, then 2 fresh mover commands: 53 total. Positive text/PAP controls, two compile refusals and four demanded failures matched. Deliberately incomplete failure fixtures emit GHC-53633 redundant-pattern warnings; no warning-free oracle claim.
- The frozen CLI passed 13 fixtures, 9 ACCEPT / 2 REFUSAL / 2 Unsupported, plus 3 isolated-PATH checks. The CLI SHA256 is 87c52e922d6cf15aefc44f09d18ef2809cfdd01203007ffdb9b44b3853df55e8, 62,862,296 bytes, mode 0555, nlink 1. Its independent retained copy depends only on system libiconv/libSystem by otool inspection.
- The unchanged original 353-file profile completed 353 attempts / 353 rows / 0 UNRUN: 65 ACCEPT / 46 REFUSAL / 242 Unsupported. Parent 761 was 65/45/243. No previously handled case moved. The sole new refusal, T9323, is confirmed by fresh GHC-83865: broken :: [Int] has RHS (). The profile omits all.T flags/ways and is not a compatibility ratio.
- Package --list returned 666 entries: 663 existing local files / 4,421,680 local source bytes and 3 generated entries. All pinned vendor sources/licences and every frozen driver file are listed. This is source inventory only, not an actual archive/install/publish test.
- Both formatting names were exercised across seven LLVM/Wasm/Cranelift refusal APIs. Unimplemented next primitives remain refused; native execution, every backend's runtime, main/B-run and release acceptance are not inferred.

Acceptance manifest:
/Volumes/ENC_4TB_WDB_CHIRHO/dev-aleluya/personal-aleluya/haskelujah-workspaces-chirho/haskelujah-next-pipeline-s0-chirho/tmp-chirho/next-show-762-chirho/checkpoint-chirho/acceptance-chirho.json
SHA256 5bc673ccc0feeb270be0dafc1fb9dec3771711d36656309bffd83dfb0965cc57

The manifest binds frozen 36-file basis 607f90ec, raw gate/Clippy/GHC/profile/package receipts and the source outgoing secret scan. Source range 36f5b03d..5b038d98 and feature metadata range 5b038d98..82884367 each passed a nonzero-range gitleaks scan. This canonical documentation/DB/report range receives its own scan before push. Required source/frozen CLI/evidence bytes remain on the owned encrypted drive; only regenerable outputs use the isolated external cache. Runtime/private TMP stays local.

## Corrections and attribution

Focused r1/r2 each ran 6 tests: 5 PASS / 1 RED. Those failures were faulty error-origin probes: program-local STG tags and the match compiler's common empty-case-on-unit fallback cannot fingerprint the original operand. Raw REDs and explicitly labelled, SHA-verified reconstructed old source remain retained. The r4 cfg(test) correction inspects ordered singleton DEFAULT cases and exact evaluated operand identities, while runtime demanded failures remain separate. These Core assertions are source-IR contract evidence, not by themselves an executed relative-demand-order demonstration. Installed optimized GHC reported p for both both-bottom probes; imprecise exceptions allow either. Nothing was rewritten to erase those negative results.

The first workspace check passed, but its Clippy command 0/judge 1 found 3 new warnings, so that packet stopped with the CLI UNRUN. R5 repaired the actual causes without suppression, then driver/workspace/Clippy/CLI gates passed on the new frozen source. No old pass is substituted for r5 execution.

Claude2 supplied the boundary in DM4689/4762/4825, read the provisional source in DM4860 and the production delta in DM4872, corrected its GHC exception prediction in DM4875, found the operand-identity test gap in DM4909, and read the four-file r5 delta in DM4960 against a 36-file hash basis. It found no changed authority/contract flaw there. All of this was source reading only; it ran no compiler, GHC, runtime or independent tests. Later CLI/profile/package/mover execution and Git custody are GPT producer evidence. An attribution-only prepublication check was requested in DM5003; no response was received at this recorded cut, so agreement is not inferred.

The CPP reader pins Show's declaration-free leading directive block and rejects directives elsewhere. Defined macro identifiers cannot occur in the chosen header/bodies. MachDeps.h/generated ghcautoconf.h names being absent from these regions remains an explicit 9.14.1 profile assumption, not general CPP support. New files are at most 338 lines. The existing shared evaluator remains 5,572 lines, with 4 routing lines added; new formatting behavior lives in separate modules. Shared instruction/root retention, quadratic Wanted rescans and earlier extension-validity residuals are unchanged.

## Resource and next-work boundary

The r5 four-phase, original 370-child profile/CLI/package and fresh two-command mover sequences each finished and actually released. DM4968/4969, 4992/4993 and 4998/4999 bind the terminal/custody facts. Per-command caps are not an aggregate watchdog. The original profile's CLI children have separate sessions and are reaped by communicate; their PGIDs were not journalled or signalled after reaping. Cargo's group is distinct from the outer capture group. Scoped empty-group/census facts never claimed global quiet or process-death coverage. The untested 1200s attempt-controller prototype was parked and never substituted into the original measurer. No compiler/native reservation remains from 762.

Next is the actual-source IO print/putStrLn vertical toward the first real main/B-run program, beginning with architectural assistance requested from Claude2 in DM5001. That request is source/design only; no new implementation, GHC/build/test reservation or IO acceptance follows from it. Show Char/Bool/list instances, full source libraries, 32-bit Int targets, native next primitives and package/runtime completeness remain open. Canonical unit 762 is closed by this metadata commit; all earlier main/native/B-run/privacy/public/release holds remain explicit.
