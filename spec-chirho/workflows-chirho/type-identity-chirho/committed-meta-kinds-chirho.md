<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Genuine metas and committed kind transport Chirho

Actual value parameter and named result producers share `fresh_value_meta_chirho`: the checked state creates a RuntimeRep meta and the value meta has kind TYPE r. An intrinsic list expression creates its element meta at the compiler List telescope's required Type parameter. Hole/error recovery, absent kind metadata and imported unknown bindings do not become Meta by this change.

```mermaid
flowchart TD
    ValueChirho[Actual value parameter or result] --> FreshChirho[Existing checked TYPE r producer]
    ListChirho[Intrinsic list expression] --> ElementChirho[Existing fresh producer at List parameter kind]
    FreshChirho --> RecordChirho[Existing owner-qualified variable record table]
    ElementChirho --> RecordChirho
    CommitChirho[Committed ordinary substitution] --> StageChirho[Preflight and stage under original owner allowance]
    RecordChirho --> StageChirho
    StageChirho --> AliasChirho[Universal dominates; Meta requires both actual Meta aliases]
    AliasChirho --> KindChirho[Existing checked telescope projection and bounded monotype relation]
    KindChirho --> ClosedChirho{Complete kind equations?}
    ClosedChirho -->|yes| ComposeChirho[Compose kind solution into ordinary substitution commit]
    ClosedChirho -->|unknown| UnknownChirho[Retain unknown image and dependent kind metadata]
    ComposeChirho --> EffectiveChirho[Replace owned mutable outward carrier with effective solution]
    EffectiveChirho --> PublishChirho[Publish complete staged records and live views with original provenance]
    UnknownChirho --> PublishChirho
    StageChirho -->|resource exhaustion| StopChirho[Sticky owner failure; no record or supply publication]
```

`apply_subst_all_chirho` takes the caller's owned mutable substitution and returns a checked `Result`. It stages the ordinary solution and additional checked kind equations before publishing either the outward carrier or its environment, equality and evidence views. Every production caller handles terminal failure explicitly, commits before composing, and retains the effective carrier or materializes its concrete checked result. A final bounded closure resolves only explicitly returned substitution domains through that carrier; it does not export the solver's working equality map or scoped givens. The strict shared relation recognizes only the existing sealed TYPE/CONSTRAINT representation equation and still uses the same actual Meta callback, budget, occurs and multiplicity checks.

The temporary record staging pass admits its full table size before traversal and charges every copied kind and equation to the same remaining owner allowance. It reads the adopted declaration state's earlier checked kind links before the new commit, without copying ordinary term equations back into that separately scoped substitution. It is not a second live registry and creates no variable IDs. Each record retains its original source owner and specificity; a chosen alias does not acquire another binder's provenance. Missing or unknown compound/scalar projection remains a decline, and an equation that the checked relation cannot close publishes no partial kind solution. Failed staging leaves the live records and supply unchanged while keeping spent work and the terminal resource failure.

Eight focused producer/transport controls cover intrinsic list metadata, a real value-result representation equation reaching the existing commit path, dependent kinds, both alias orientations under Universal/Unknown/absent barriers, foreign provenance, unknown compound free variables, atomic owner-budget exhaustion and strict representation equations in both orientations. They are source controls, not a repair claim for the original fundep witnesses. Their execution is scheduled by the root compiler owner.

```mermaid
flowchart TD
    EntryChirho[Before local givens or rigid opening] --> CaptureChirho[Consume actual owner checked outer records and kind dependencies]
    CaptureChirho --> InnerChirho[Infer under private local evidence]
    InnerChirho --> ProjectChirho[Existing scope projection of older public roots]
    ProjectChirho --> StagePublicChirho[One shared transport stage with pre-entry record base]
    StagePublicChirho --> ValidateChirho[Check complete effective solution and dependent kinds for private escape]
    ValidateChirho -->|closed| PublishPublicChirho[Publish the same effective carrier and materialized public records]
    ValidateChirho -->|escape or resource failure| RollbackChirho[Consumed prepaid capture restores outer records]
    RollbackChirho --> TerminalChirho[One non-deferrable failure; no fresh recovery variable]
```

The opaque pre-entry capture retains actual record presence, role, owner, origin and specificity, with kinds resolved under pre-local checked links. It is temporary consumed scope ownership, not a second catalogue. The public stage overlays only those original outer permissions before the existing bounded transport; it must not re-read later private declaration links for captured kinds. Public links preserve Universal/Unknown/absence barriers, while a private alias alone does not permanently remove an original Meta permission. Complete staged validation covers newly derived kind bindings as well as the input projection. Additional domains require the bounded dependency graph rooted in explicit public inputs, their images and actual checked kind edges; graph-unrelated domains and actual private rigid images are refused before publication. New image variables use their real pre-publication kind edges, before staging can erase a solved representation variable. This graph admits no binding authority: kind equations still require the existing checked Meta-only stage. Failure restores only captured keys using work and storage reserved before entry, preserves spent work and never resets the owner's budget or allocates a recovery meta. Concrete signature and constructor-pattern controls exercise kind-only escape, safe outer permission recovery, full-effective-delta validation and terminal public rejection. Source controls and formatting alone do not establish executed acceptance.

The initial age-only additional-domain restriction and its negative oracle were overbroad. A positively checked fresh inference value of kind TYPE r can safely resolve r to Lifted even when created after scope capture. The paired correction preserves that old setup as a positive, and the existing negative now uses a genuinely local rigid RuntimeRep binder opened by the actual signature route, together with a new inferred representation equation. It must refuse specifically for the private rigid image before publishing either equation. This changes the incorrect permission assumption; it does not waive the Universal/Unknown, owner, resource, whole-tree closure or single-publication obligations. The corrected pair awaits its own executed gate.
