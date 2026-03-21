# Lazy Evaluation Design — Haskelujah Chirho

## Status: Design Complete, Implementation Starting

## Key Architecture

### Thunk Layout (heap object)
```
[0..8]   header: (code_ptr << 2) | 0b00  (ThunkChirho tag)
[8..8+N] free variables: [fv_0, fv_1, ..., fv_N]
```

### After evaluation (updated in-place)
```
[0..8]   header: (result_ptr << 2) | 0b01  (indirection)
```

### Object Kind Bits (low 2 of header)
- 00 = Thunk
- 01 = Fun
- 10 = Con
- 11 = PAP/IND/Blackhole (extended)

## RTS Functions Needed
1. `haskelujah_alloc_thunk_chirho(code_ptr, num_fvs, fvs_array) -> thunk_ptr`
2. `haskelujah_enter_thunk_chirho(thunk_ptr) -> result`
3. `haskelujah_update_thunk_chirho(thunk_ptr, result_ptr)`

## GC Root Functions (already exist, need wiring)
- `haskelujah_gc_root_push_chirho(ptr)` — declared in Cranelift codegen
- `haskelujah_gc_root_pop_chirho()` — declared in Cranelift codegen

## Implementation Phases
1. RTS thunk ops (1-2 days)
2. Core IR ThunkChirho variant (1 day)
3. Cranelift codegen (2-3 days)
4. LLVM codegen (1-2 days)
5. Strictness analysis (1 day, optional)
6. GC integration (1 day)
7. Testing (2-3 days)
