<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->
# How a native executable fails Chirho

Row 535 (P1). Where the interpreter names a failure, a native executable either is refused at compile time or fails the same named way: it flushes stdout, writes exactly one stderr line, and exits 1. It never aliases an enclosing parameter, substitutes 0, formats a heap pointer as an integer, or dies by `abort()` or a signal.

```mermaid
flowchart TD
    core_chirho["Prepared Core: IO actions and native thunks"] --> llvm_chirho[LLVM codegen]
    core_chirho --> cranelift_chirho[Cranelift lowering]
    llvm_chirho -->|primitive it cannot lower| refuse_chirho["Refused: '&lt;backend&gt; does not support primitive(s): name/arity, ...'; the CLI exits 1"]
    cranelift_chirho -->|primitive it cannot lower, unless a call site implements it| refuse_chirho
    llvm_chirho -->|free id nothing binds| unbound_chirho["haskelujah_unbound_thunk_chirho: one rooted thunk per name"]
    cranelift_chirho -->|free id the pre-scan named| unbound_chirho
    unbound_chirho -->|demanded| route_chirho
    unbound_chirho -->|never demanded| runs_chirho[The program runs as before]
    errors_chirho[error, errorWithoutStackTrace, pattern failure, missing method, IO fail] --> route_chirho
    undefined_chirho[undefined] --> route_chirho
    throw_chirho["Cranelift throw#: uncaught exception"] --> route_chirho
    nomatch_chirho[A case default with no alternative] --> route_chirho
    boxed_chirho[show_int, print_int or an Int list head given a live heap value] --> route_chirho
    invariants_chirho[Allocator, main thread and thunk invariants] --> internal_chirho
    route_chirho["fail_chirho: flush Rust stdout and C stdio, one line, exit 1"] --> user_chirho["haskelujah: error: message"]
    internal_chirho["fail_chirho, internal prefix"] --> internal_line_chirho["haskelujah: internal error: message"]
```

## Where each step lives

- The route: `haskelujah-rts-chirho/src/ffi_chirho/failure_chirho.rs`. The user and internal prefixes are the classification. A blackholed thunk reports `<<loop>>`.
- LLVM: `codegen_chirho/failures_chirho.rs` records each function's unbound free ids and emits the thunk where one is referenced. It also writes the `error` and `undefined` bodies and the no-match defaults. Int `show` and a pure `main`'s result call the checked runtime formatters.
- Cranelift: `codegen_chirho/failures_chirho.rs` names the unbound ids in the pre-scan and refuses recorded primitives. `lower_chirho/failures_chirho.rs` emits the thunk and records refusals. `codegen_chirho/runtime_calls_chirho.rs` imports the runtime entries.
- A refusal fires on any primitive in a compiled body, called or not. A reachable Prelude `Ord` dictionary compiles `compare#` and `compareChar#`, so Cranelift lowers both as LLVM does (`lower_chirho/ordering_chirho.rs`: LT, EQ and GT tags). Row 535's gate r1 was refused without them.
- Row 538 removes integer `^#` from the refused set. LLVM and Cranelift both call `haskelujah_pow_int_chirho`, the RTS implementation of the interpreter's wrapping integer-power convention. The P06 driver control runs once through the interpreter and both native backends and requires GHC's `1024`.
- Row 540 removes `fromIntegral#` from the native refused set. LLVM and Cranelift both convert signed `i64` to `double`, then bitcast the result into the backends' uniform `i64` value representation. The saved P03 control requires `3.5` through the interpreter and both native backends.
- Row 539 removes `compareFloat#` and `compareStr#` from the refused set. Both backends now lower all four compare primitives through one Ordering selection: less gives LT, equal gives EQ, and anything else gives GT.
  - LLVM does this in `emit_compare_chirho`, and Cranelift in `lower_chirho/ordering_chirho.rs`.
  - Double uses ordered tests, so a NaN gives GT and -0.0 equals 0.0, as on GHC.
  - String calls C `strcmp`, through the import `eqStr#` already uses. Its unsigned bytes order UTF-8 by code point.
  - Without the `strcmp` import, Cranelift refuses `compareStr#`.
  - Driver controls:
    - Compare2 requires GHC's `GT GT LT GT` through the interpreter and both native backends.
    - A GHC-measured edge program covers NaN, -0.0, the empty string, a prefix, non-ASCII and a computed `show`. Row 542's interpreter correction promotes the same control to all three engines.
- Row 541 removes `newIORef#`, `readIORef#`, and `writeIORef#` from the native refused set. Both backends call the same RTS allocation-kind-checked mutable cell implementation; its initial payload is rooted across allocation and remains visible to native GC traversal. The saved P01 control requires `2` through the interpreter and both native backends.
- Row 544 removes `toUpper#` from LLVM's refused set and routes both native backends through `haskelujah_char_to_upper_chirho`, which shares the interpreter's first-scalar Unicode convention. Native String output has two representations: aligned NUL-terminated buffers for literals and `show`, and registered low-bit-tagged `[Char]` cells after list operations. `haskelujah_put_str_ln_chirho` packs only the tagged-list form before printing. The saved P02 control requires `ABC` through the interpreter and both native backends.
- Row 545 removes `mapFromList#` and `mapSize#` from the refused set only for the representation-safe boundary. The shared RTS validates a native association-list spine, copies an empty or singleton `fromList`, and counts it without inspecting key bits. A second pair fails named because generated Core has erased `Ord k`; treating every key as signed `i64` would silently misorder legal `Double` maps. `mapToList#` remains a compile-time refusal, so P04 stays closed while saved P05 requires `1` through the interpreter and both native backends.
- Through the CLI, a Cranelift refusal reads `cranelift compilation failed: Cranelift does not support primitive(s): …`, because the CLI wraps every Cranelift compile error (`haskelujah-cli-chirho/src/main.rs:816`). LLVM's refusal prints bare. The driver controls call the backends directly, so they see the bare line on both.

## Residuals outside the route

- A Cranelift miss for a local that Core binds but lowering did not bind keeps its 0 placeholder, like letrec zero initialization.
- An effect adapter called as a value, such as `putStrLn#` passed to `mapM_`, keeps Cranelift's call-site behavior.
- Memory faults from upstream type confusion stay outside the route.
  - K01's Cranelift run, a SIGBUS at the base, now fails named with R2's line (measured at gate r3).
  - The likely reason, which is an inference and not an established cause: its `show` path reaches the checked runtime formatter before the old fault.

## Controls

- RTS (`haskelujah-rts-chirho/tests/native_failure_chirho.rs`): each case runs in a child process and must exit 1 with its exact line. The report must follow buffered C output on one stream. Cases: user, internal, unbound, boxed, no-match and loop.
- LLVM unit tests:
  - a demanded unbound id fails named, and an untaken one runs;
  - Int `show` and the pure `main` result use the checked formatters.
- Driver (`haskelujah-driver-chirho/tests/native_error_helpers_chirho.rs`): one test per route per backend, with exact harness text. The shared error-helper programs require exit 1 and their exact line.
