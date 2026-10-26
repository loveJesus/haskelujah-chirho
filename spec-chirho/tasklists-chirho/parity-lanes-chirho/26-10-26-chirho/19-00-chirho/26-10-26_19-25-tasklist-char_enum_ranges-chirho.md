<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Row 569: Char Enum bounded ranges

Base: `f8b3476a`, immediately after row 567 closed and row 569 was reserved.

- [x] Claim the interpreter-only Char range residual isolated by row 567.
- [x] Measure ascending, descending, stepped, empty and variable-bound ranges on GHC 9.14.1 and current main.
- [x] Trace the generated sequence path before editing source.
- [x] Preserve the current Enum value's Int/Char runtime kind at each bounded recursive step.
- [x] Add all-engine controls and a mutation-sensitive mechanism check.
- [x] Run the warm focused gate, changed-line Clippy, rustfmt and diff-check; run gitleaks on the committed range before landing.
- [x] Rebase onto exact main, rerun the controls, land, and close only row 569.

GHC prints `"abcde"`, `[97,98,99,100,101]`, `'e'`, `"bcd"`, `"eca"`,
`"ace"`, `""` and `"a"` for the measured interpreter probe. The interpreter prints
the first literal as `Char` and every generated successor as `Int`:
`['a',98,99,100,101]`; `last ['a'..'e']` is therefore `101`, and a variable-bound
range is mixed too. A later direct native check corrected the initial native claim: printing
the range as a String fails by name on both LLVM and Cranelift, at the separate native
Show-String boundary owned by rows 560/557.

The root is the generic Int-shaped `enumFromTo`/`enumFromThenTo` Core binding. Its first
element keeps the source literal's `Char` value, but its recursive `+#`/arithmetic step returns
an `Int` in the interpreter. The first attempted repair routed an evidenced Char range through
a second helper. Its interpreter result was right, but both native backends then printed the
runtime-built String as empty, crossing rows 560/557. That approach was removed.

The bounded generic bindings now advance with `enumStepChirho#`. In the interpreter it returns
the same runtime kind as its current value: Int plus a step is Int; Char plus a step is Char.
LLVM, Cranelift and Wasm lower it as the same integer addition native Char already uses. The
all-engine control observes explicitly typed Char ranges through `map fromEnum` and `last`, so
it proves the element representation without claiming native String rendering. It covers
ascending, descending, stepped, empty, variable-bound, explicit `enumFromTo` and
`enumFromThenTo`, plus an unchanged Int range.

Unbounded ranges keep their existing `+#` step. Parser escapes, String representation,
`succ`/`pred`, surrogate values, derived Enum sequences and other Enum keys remain outside
row 569.

The warm focused gate is GREEN: 320 runtime/core/LLVM/Cranelift/Wasm library tests,
52 driver primitive/Enum/STG-lowering library tests, and all 99 dictionary-evidence tests.
The all-engine Row 569 control passed inside that final suite. Clippy completed on all six
changed crates with no diagnostic on a changed line; rustfmt and `git diff --check` are clean.
The existing warnings reported elsewhere in the dependency closure are not attributed to this
row.

The mechanism mutation replaced the two bounded `enumStepChirho#` recursive advances with
their old `+#` form. The all-engine control failed at `last ['a'..'e']`: the interpreter again
returned `101` instead of `'e'`. Restoring both sites byte for byte and rebuilding made the
control pass. This proves the new step, rather than the annotations in the control, carries the
Char representation through recursion.

The source commit rebased cleanly onto exact main `8b5b09db` as `44a48156`. After the rebase,
the runtime primitive control passed 1/1 and the all-engine bounded-range control passed 1/1;
`git diff --check` stayed clean and gitleaks found no leaks in the one-commit range. Main then
fast-forwarded to `44a48156`. Row 569 alone is closed; the named String-rendering, escape,
unbounded-range and surrogate boundaries remain open in their own rows.
