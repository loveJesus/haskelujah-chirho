<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 601: Result-only Enum propagation Chirho

- [x] Reserve Row 601 in the canonical progress database on `main_chirho`.
- [x] Measure the result-only `Enum a` wrapper on GHC 9.14.1 and the interpreter, LLVM and real Cranelift.
- [x] Trace its checker evidence, retained dictionary parameters and call-site dictionary arguments.
- [x] Post a bounded design checkpoint and overlap audit before compiler source changes.
- [x] Implement only the proven result-type evidence repair with structural and all-engine controls.
- [x] Mutation-prove the repair and run the focused warm gate.
- [x] Land through the canonical single-writer chain and close Row 601.

## Scope Chirho

- The mover begins with `convertEnumChirho :: Enum a => Int -> a; convertEnumChirho = toEnum`, instantiated at `Char` and `Int`.
- Directly annotated `toEnum`, witnessed `Enum` wrappers and Row 599's result-only `Num` route remain holds.
- Rows 597 and 598 own generated and qualified name resolution. Their files and mechanisms stay outside this row.
- Reservation base: `861ea7e7`.

## Baseline and trace Chirho

- GHC 9.14.1 prints `'a'`, `42`, `"ab"` and `[40,41]` for the Char and Int direct and mapped forms.
- The interpreter prints `97` and `42`, then stops at `ShowStrChirho: expected Char, got 97#`. LLVM and marker-verified real `--cranelift` happen to match GHC because native Int and Char share a word representation in these cases.
- The free `toEnum` method occurrence already carries exact `(Enum, $own:0)` occurrence evidence. Row 599's shared parameter inventory inspected only constrained-reference evidence for `Num`, so it never saw this separate method-occurrence proof; the wrapper was bound to the default `$fEnumInt` route and received no Enum dictionary.

## Bounded repair Chirho

- Admit `Enum` through the method-occurrence proof channel beside `Num`'s constrained-reference channel. The matching class and exact `$own:<predicate-index>` remain mandatory.
- Definition transformation and caller pre-seeding already use the same body-aware inventory; there is no checker, backend, representation or general name-resolution change.
- Missing, wrong-index or wrong-class evidence retains today's default. Directly annotated and witnessed Enum routes plus Row 599's Num route remain holds.
- The design checkpoint was posted as Metropoleluya message `#29687` before source changes.
- `Enum Ordering` is a pre-existing typing gap (`no instance for Enum Ordering`) and remains outside this result-propagation row; the first candidate control exposed it before the focused gate.

## Source checkpoint Chirho

- `binding_dict_param_indices_chirho` now reads `Enum`'s exact body proof from method-occurrence evidence while leaving `Num` on Row 599's constrained-reference evidence channel.
- The structural control accepts `(Enum, $own:1)` for predicate index 1 and rejects `(Enum, $own:0)` there.
- The all-engine control covers Char, Int, Bool, a derived Enum and mapped Char/Int uses. Each is byte-identical to GHC on the interpreter, LLVM and marker-verified real Cranelift.
- The corrected trace and source checkpoint were posted as Metropoleluya message `#29695`; all temporary instrumentation was removed.

## Reach and gate r1 Chirho

- Removing only the method-occurrence proof channel makes the structural control return `[0]` instead of `[0,1]` and makes the all-engine mover fail in the interpreter with the old derived-Enum tag error. Both failures are named; Row 599's reference-evidence channel is untouched.
- The source restored byte-identically at SHA-256 `0bcca1610692e3ddb393c19550a752ce9fb57fea53c73072790952215aa6e129`, and both controls passed again.
- Gate r1 is GREEN on source commit `154c8df8`. Build completed with no warning; core passed 187/187, dictionary evidence passed 118/118, and the focused driver evidence/Enum slice passed 59/59: 364 selected tests in 121 seconds.
- Clippy reported only three pre-existing findings on untouched lines 161, 314 and 318 of `dict_params_chirho.rs`; no changed line has a diagnostic. Rustfmt, diff-check and gitleaks are clean. The lint and hygiene phase took about 17 seconds.

## Landing Chirho

- Source commit `154c8df8` and gate-record tip `bc2df4d4` fast-forwarded onto canonical `main_chirho`.
- The progress database closure commit records Row 601 complete after its integrity check.
