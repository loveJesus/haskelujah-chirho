# Lazy Evaluation Design — Haskelujah Chirho

## Status: Infrastructure Complete, Thunk Creation Next

## Key Architecture

### Thunk Layout (heap object)
```
[0..8]   header: (code_ptr << 8) | state_bits[7:4] | kind_bits[1:0]
[8..8+N] free variables: [fv_0, fv_1, ..., fv_N]
```

### Header Bit Layout
- Bits [63..8]: code pointer (unevaluated) or result value (indirection)
- Bits [7..4]:  thunk state (0=unevaluated, 1=blackhole, 2=indirection)
- Bits [3..2]:  GC mark bits
- Bits [1..0]:  object kind (00=thunk, 01=fun, 10=con, 11=pap)

### Heap Pointer Tagging
All heap pointers have bit 63 set. `enter_thunk` checks this first —
unboxed values (ints, chars, tags) have bit 63 clear and are returned as-is.

## RTS Functions (DONE)
1. `haskelujah_alloc_thunk_chirho(code_ptr, num_fvs, fvs_array) -> thunk_ptr` ✅
2. `haskelujah_enter_thunk_chirho(thunk_ptr) -> result` ✅ (with bit-63 guard)
3. `haskelujah_update_thunk_chirho(thunk_ptr, result_ptr)` ✅

## GC Root Functions (WIRED)
- `haskelujah_gc_root_push_chirho(ptr)` — called after Con/PAP allocs ✅
- `haskelujah_gc_root_pop_chirho()` — declared, called from LLVM ✅

## Thunk Trampoline Functions (DONE)
Per-arity trampolines (0..8) in Cranelift object module:
- Signature: `fn(fvs_ptr: i64) -> i64`
- fvs layout: `[func_ptr, arg0, arg1, ..., argN]`
- Loads func_ptr + args from fvs, calls indirectly, returns result
- Used as `code_ptr` in `alloc_thunk` for known function application thunks

## Case Scrutinee Forcing (DONE)
Both Cranelift and LLVM backends call `enter_thunk` on every case
scrutinee value. Since case is the only strict evaluation point in
Haskell, this ensures thunks are forced at the right places.
`enter_thunk` is a no-op for non-thunks (just checks bit 63).

## Implementation Phases

### Phase 1: RTS thunk ops ✅
- alloc_thunk, enter_thunk, update_thunk in ffi_chirho.rs
- Bit-63 heap pointer guard to avoid dereferencing unboxed values

### Phase 2: Core IR — SKIPPED
Core stays strict (like GHC Core). Codegen decides when to thunk.

### Phase 3: Cranelift codegen — IN PROGRESS
- ✅ Wire alloc_thunk, enter_thunk, gc_root_push/pop FuncRefs to LowerCtxChirho
- ✅ Generate thunk trampoline functions (arities 0-8)
- ✅ GC root push after constructor and PAP allocations
- ✅ Force thunks at case scrutinees
- ✅ force_if_thunk_chirho helper for future primop forcing
- ✅ collect_app_chain_chirho + try_create_thunk_for_app_chirho infrastructure
- 🔜 Enable thunk creation for non-recursive let-bound function applications
- 🔜 Add primop argument forcing (once thunk creation is active)

### Phase 4: LLVM codegen
- ✅ Declare alloc_thunk, enter_thunk in LLVM IR
- ✅ Force thunks at case scrutinees (both expr + tail positions)
- ✅ GC root push/pop around allocations
- ✅ emit_force_thunk_chirho helper ready
- 🔜 Mirror thunk creation from Cranelift

### Phase 5: Strictness analysis (optional optimization)
- Avoid creating thunks for values that will always be forced
- Demand analysis pass on Core IR

### Phase 6: GC integration
- ✅ gc_root_push after heap allocations
- 🔜 Lower GC_THRESHOLD_CHIRHO from u64::MAX once root tracking is complete
- 🔜 Test GC with long-running programs

### Phase 7: Testing
- 🔜 End-to-end thunk round-trip test
- 🔜 Infinite list test (take 5 (repeat x))
- 🔜 Sharing test (let x = expensive in x + x)
