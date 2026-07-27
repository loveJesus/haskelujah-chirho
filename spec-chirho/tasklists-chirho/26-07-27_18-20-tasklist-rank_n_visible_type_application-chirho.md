<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Rank-N visible type application tasklist

Owner: `gpt_chirho`

## Brick 1

- [x] Reproduce the deterministic `T17594f.hs` mismatch at `f @Int 14`.
- [x] Identify the representation boundary: rank-N parameter quantifiers remain inside
      `TyChirho::ForallChirho`, while named VTA currently consumes only scheme variables.
- [x] Pin nested-`forall` VTA with a focused inference unit test.
- [x] Trace the next blockers in the exact fixture: mixed term/type application spines,
      required `forall a ->` binders, and visible type abstraction.

## Implementation

- [x] Instantiate outer nested `forall` binders after ordinary scheme variables, preserving
      source order and partial visible application behavior.
- [x] Preserve required foralls in the CST, AST, internal type, substitution, unification,
      kind inference, validity checking, and record-field reconstruction.
- [x] Preserve `\ @a -> body` as a type lambda through lowering and Core desugaring,
      including kind-annotated binders.
- [x] Consume required type arguments through ordinary applications and invisible type
      arguments through `@T` applications without skipping a function arrow.
- [x] Match required declaration arguments against signatures without turning them into term
      function arrows.
- [x] Keep existing top-level VTA, wildcard VTA, and scoped-variable behavior unchanged.
- [x] Make `T17594f.hs` pass.
- [x] Record the separate minimal rank-N stack-overflow observation without conflating it with
      the deterministic `T17594f` mismatch.

## Gates

- [x] Run focused VTA, required-forall, type-lambda, application-spine, and `T17594f` tests.
- [x] Run the typing suite (`262 passed`, `1 ignored`) and Core suite (`128 passed`).
- [x] Run the deterministic parser surface (`289 passed`, `3 ignored`) while excluding the
      two known preprocessed-fixture failures and the known nested-parentheses proptest overflow.
- [x] Run bounded representative GHC VTA checks and a warning-free CLI build.
- [x] Run formatting checks and `git diff --check`.
- [x] Commit explicit owned paths, push `main_chirho`, release the builder, and request a
      measured corpus update only if a tracked file flips.

## Residual boundary

- The minimized standalone rank-N/VTA stress shape can still overflow the checker stack even
  though the exact `T17594f.hs` driver path is green. It remains a separate hardening issue and
  is not counted as fixed by this slice.
- Named required declaration binders are consumed positionally; this slice does not yet expose
  them as scoped type names in the equation body.
