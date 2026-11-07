<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Error deferral Chirho

Row 695. A typed hole is an error by default, as in GHC (GHC-88464). Three independent GHC switches can turn particular inference errors into warnings. Every other error stops the module.

```mermaid
flowchart TD
    caller_chirho["caller options<br/>FrontendOptionsChirho"] --> policy_chirho
    header_chirho["real header LANGUAGE and OPTIONS_GHC<br/>HeaderPragmasChirho<br/>source path: original, then post-CPP"] --> policy_chirho
    policy_chirho["FrontendPolicyChirho<br/>caller first, then both real header views"]
    policy_chirho --> extensions_chirho["EffectiveExtensionsChirho<br/>last edition baseline, explicit overrides<br/>one effective state per extension"]
    extensions_chirho --> parse_chirho["configured parser before layout<br/>configured lower publishes Module.extensions<br/>splices and deriving"]
    parse_chirho --> resolve_chirho["name resolution"]
    resolve_chirho -->|any error| stop_chirho["reject with the phase's bundle"]
    resolve_chirho --> kind_chirho["kind inference"]
    kind_chirho -->|any error| stop_chirho
    kind_chirho --> infer_chirho["type inference"]
    infer_chirho --> settle_chirho{"settle_inference_chirho<br/>is every error deferred?"}
    policy_chirho --> deferral_chirho["existing DeferralPolicyChirho categories"] --> settle_chirho
    settle_chirho -->|no| stop_chirho
    settle_chirho -->|yes: those errors become warnings| validity_chirho["source-type validity"]
    validity_chirho -->|any error| stop_chirho
    validity_chirho --> exhaust_chirho["exhaustiveness"] --> result_chirho["FrontendResultChirho with warnings"]
```

## Rules

- `-fdefer-type-errors` turns on all three switches: type errors, typed holes and out-of-scope variables.
- `-fno-defer-type-errors` turns off only the general switch.
- `-fdefer-typed-holes` and `-fdefer-out-of-scope-variables`, and their `-fno-` forms, change only their own category.
- Options apply left to right. With `-fdefer-type-errors -fno-defer-typed-holes` (T9497d) the hole stays an error. With the reverse order it defers.
- The caller's options come first, then the module header's `OPTIONS_GHC` (and legacy `OPTIONS`) pragmas.
- Header pragmas come from real pragma tokens, read only until the first token that is neither a pragma nor a comment. A flag in a comment, in a string literal, or in a pragma after `module` is not an option.
- On the source path that runs CPP (`typecheck_source`, `compile_source`, `eval_source`), the header is read twice, both times after the caller's options:
  - first as written, before preprocessing, so CPP cannot drop one of its options;
  - then after preprocessing, as GHC re-reads the pragmas once CPP has run, so an `OPTIONS_GHC` that CPP selects applies too.
- Direct frontend callers read the header of the text they pass in.
- Extension resolution uses GHC 9.14.1's default GHC2021 baseline. The last named language edition replaces that baseline; every explicitly enabled or disabled extension overrides it in written order, including flags written before the edition. Explicit enabling replays GHC's implication graph; the edition baseline does not take that closure. Thus GHC2021's implied `TypeOperators` leaves `ExplicitNamespaces` off, while an explicit `-XTypeOperators` enables it.
- Documented extension aliases, including `DoRec`, `PatternSignatures`, `Rank2Types`, `PolymorphicComponents`, `RecordPuns`, `GeneralisedNewtypeDeriving` and their `No` forms, normalize into the same canonical flag state before implication replay. Ordinary scoped-variable inference reads that canonical state; `TypeAbstractions` does not invent a `ScopedTypeVariables` implication.
- `EffectiveExtensionsChirho` supplies both contextual keyword classification before layout and the lowered module's extension field. Default parser/lower entry points use the same catalogue, including cold discovery. A caller's `-XCPP` controls the initial CPP decision; real `NoCPP`, comments, strings and late pragmas follow the same header authority.
- `NoPolyKinds` refuses written kind variables and kind quantifiers. Already checked hidden kind arguments retain their producer-owned variable identities; disabling source syntax does not invalidate those checked records.
- Option words follow GHC's `toArgs`:
  - A word that starts with `"` is a Haskell string literal, read without its quotes. Only `\"` and `\\` are decoded.
  - A quoted part inside a word keeps its quotes and may hold spaces.
  - Single quotes are ordinary characters: GHC accepts `"-fdefer-typed-holes"` and rejects `'-fdefer-typed-holes'`.
  - A body GHC cannot read, such as an unterminated or misplaced quote, gives no options.
- An unbound unqualified `_` or `_name` in an expression is a hole (E0210).
  - A bound `_name` is an ordinary variable.
  - A qualified spelling is an unbound variable.

## What can defer

Only inference errors whose code the typing crate names as deferrable:

| Switch | Codes |
|---|---|
| typed holes | E0210 |
| out-of-scope variables | E0202 (unbound variable) |
| type errors | E0200 mismatch, E0201 occurs check, E0203 tuple arity, E0204 unsatisfied constraint, E0205 signature mismatch |

Other errors stay fatal under every switch:

- **Inference codes:**
  - E0206 record field;
  - E0207 strict field;
  - E0208 pattern signature;
  - E0211 unbound record constructor;
  - E0212 duplicate instance;
  - E0213 signature-converter invariant;
  - E0214 `RecordWildCards` field with no variable in scope;
  - uncoded errors.
- **Whole phases:** name resolution, kind inference, declaration and source-type validity, and exhaustiveness.

E0211 to E0214 were separated from the deferrable E0202, E0204, E0205 and E0200, because no GHC switch defers any of them.

## Boundaries

- **Qualified out-of-scope names:** a qualified out-of-scope name (`Q.x`, name resolution's E0100) never defers. GHC defers such a variable under `-fdefer-out-of-scope-variables`, but E0100 also covers data constructors, which GHC never defers.
- **CPP-selected options:** a pragma that only a header `#if` selects is read from the preprocessed text alone. The initial real header and caller options decide whether CPP runs; the selected header then contributes to the final policy.
- **Prepared file transport:** older file/project readers still return only preprocessed strings. Carrying the original-header policy through those existing file and batch containers remains required; the shared configured parser/lower API does not itself establish that every older file caller retains both views.
- **Option word forms not modeled:**
  - the `["..."]` list form that GHC also reads;
  - Haskell escapes other than `\"` and `\\`.
- **Unreadable pragmas:** a malformed or unknown option does not reject the module as GHC does; it is simply not a switch.
- **No runtime claim:** a deferred module compiles. Evaluating a deferred hole, unbound variable or type error does not raise GHC's deferred runtime error; it does whatever the existing lowering does.
- **Callers without options:**
  - The CLI, multi-module and dependency-package builds pass no caller options; only the module header's pragmas apply there.
  - Warning-only flags such as `-fno-warn-typed-holes`, and `-Werror`, are not modeled.
- **Corpus harness:** the directory-label corpus harness applies no `all.T` options. Should-compile hole tests whose `all.T` passes `-fdefer-typed-holes` or `-fdefer-type-errors` become nominal rejects there. That is a harness boundary, not compiler behavior.

The shared edition tables and explicit implication graph follow [GHC 9.14.1 DynFlags](https://github.com/ghc/ghc/blob/ghc-9.14.1-release/compiler/GHC/Driver/DynFlags.hs#L1205) and [Flags](https://github.com/ghc/ghc/blob/ghc-9.14.1-release/compiler/GHC/Driver/Flags.hs#L274). Language changes replay explicit flags as described by [Session](https://github.com/ghc/ghc/blob/ghc-9.14.1-release/compiler/GHC/Driver/Session.hs#L2794).

## Controls

- `pragma_chirho/effective_tests_chirho.rs`: default/selected editions, explicit override precedence, implication versus baseline distinction, real header boundaries, one published flag state and the shared pre-layout/lowering payload.
- `source_chirho/language_options_tests_chirho.rs`: actual kind elaboration under caller modes, contextual layout, caller-enabled CPP, quoted/real headers, ordinary deferral categories, and bound/free versus concrete kinds under `NoPolyKinds`. The corresponding twelve fresh GHC controls preserve their pinned predictions and raw receipts.

- `tests_chirho/extensions_chirho/deferral_flags_chirho.rs`: flag implications, both orders, caller-then-module precedence, unrelated options, header option words, quoted option words, and fake flags in comments, strings and misplaced pragmas.
- `tests_chirho/extensions_chirho/typed_hole_policy_chirho.rs`: end-to-end checks on typed `main :: IO ()` programs:
  - default `_` and `_missing` rejection;
  - bound `_x`;
  - deferred unused-hole execution;
  - both flag orders;
  - module-over-caller precedence;
  - fake flags;
  - category separation;
  - kind, duplicate-instance and record-constructor errors that stay fatal;
  - a CPP module's header options on the source path;
  - double- versus single-quoted switches.
- The three earlier `typed_hole_*` tests in `extensions_chirho.rs` keep their sources and expectations and now ask for `-fdefer-typed-holes` explicitly. Their unsigned `main = 42` keeps any inherited row 692 numeric-entry failure as it was.

## Checked consumer search options Chirho

`compile_source_with_search_path_and_options_chirho` applies caller options to the checked consumer while retaining the same filesystem sibling/hierarchical discovery and reached-module admission. The existing `compile_source_with_search_path_chirho` and file-oriented entry keep their default-options behavior. Providers retain their own source headers and default route; caller flags are not propagated session-wide. This does not add CPP preprocessing or a Program/Value/IO-entry contract.

The existing `FrontendPolicyChirho` resolves caller options before the actual consumer header. That policy reaches BOTH the early discovery parser/lowerer and the final existing option-aware frontend. Interfaces, inferred types, synonyms, families and the fresh empty imported class environment follow the same producer and backend path; tests do not reconstruct certificate-bearing interfaces.

```mermaid
flowchart TD
    caller_options_chirho[Checked consumer caller options] --> actual_header_chirho[Then the actual consumer header]
    actual_header_chirho --> resolved_policy_chirho[Existing FrontendPolicy]
    resolved_policy_chirho --> root_discovery_chirho[Root discovery parser and lowerer]
    root_discovery_chirho --> existing_admission_chirho[Existing reached-module admission]
    provider_headers_chirho[Provider own headers and default route] --> existing_admission_chirho
    existing_admission_chirho --> existing_frontend_chirho[Same imports into option-aware frontend]
    resolved_policy_chirho --> existing_frontend_chirho
    existing_frontend_chirho --> existing_backend_chirho[Same compilation result and backend]
```
