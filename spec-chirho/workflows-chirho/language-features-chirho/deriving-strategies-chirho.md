<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth
in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Deriving strategies workflow

Row 694 (D01). `newtype AgeChirho = AgeChirho Int deriving newtype Show` must print `7`, as
GHC 9.14.1 does, never the stock `AgeChirho 7`. A deriving clause therefore keeps its
strategy for every class it lists, an explicit strategy chooses the deriving mechanism, and a
newtype that takes a class through the newtype strategy (GND) shares its representation's
dictionary and its representation itself.

```mermaid
flowchart TD
    cst_chirho["CST data_decls_chirho.rs: any number of deriving clauses; a strategy is stock or anyclass (identifiers) or the newtype keyword token"]
    cst_chirho --> lower_chirho["Lowering deriving_clauses_chirho.rs: one DerivedClassChirho per class, carrying its clause's DerivingStrategyChirho; via clauses stay in deriving_via_chirho"]
    standalone_chirho["CST and lowering: deriving [strategy] instance; StandaloneDerivingDeclChirho.strategy_chirho"]
    pragmas_chirho["Parser module_extensions_chirho: each written flag after every edition; GHC2024 adds DerivingStrategies"] --> policy_chirho["extension_policy_chirho.rs DerivingExtensionsChirho"]
    policy_chirho --> validity_chirho
    policy_chirho --> route_chirho
    lower_chirho --> validity_chirho["Declaration validity deriving_strategy_chirho.rs, rules 7-12: GHC-87139, 10333, 26557, 38178, 00158, 82023"]
    standalone_chirho --> validity_chirho
    lower_chirho --> route_chirho["strategy_route_chirho.rs DerivingPolicyChirho::route_chirho: written stock, anyclass or newtype; an unwritten strategy keeps the legacy policy"]
    standalone_chirho --> route_chirho
    scope_chirho["Pipeline: build_builtin_class_scope_chirho before deriving (K01)"] --> route_chirho
    route_chirho -->|stock| stock_chirho["declaration_heads_chirho.rs derive_stock_class_chirho: the class's generator"]
    route_chirho -->|anyclass| empty_chirho["derive_anyclass_chirho: an empty instance using the class defaults"]
    route_chirho -->|newtype| gnd_chirho["newtype_route_chirho.rs derive_newtype_instance_chirho: C rep => C N, eta-reduced by the builtin class's parameter arity, no methods"]
    route_chirho --> inventory_chirho["newtype_strategy_classes_chirho: each newtype's newtype-routed classes"]
    stock_chirho --> metadata_chirho["Show and Enum metadata: only entries that name the class with no strategy or stock"]
    inventory_chirho --> info_chirho["Driver build_newtype_info_chirho: NewtypeInfoChirho.newtype_derived_classes_chirho"]
    info_chirho --> eligible_chirho["Dict pass newtype_dicts_chirho.rs takes_representation_dictionary_chirho: the declaration is the only proof"]
    gnd_chirho --> eligible_chirho
    eligible_chirho --> ground_skip_chirho["generate_instance_dicts_chirho skips it"]
    eligible_chirho --> alias_chirho["generate_gnd_dicts_chirho: each method row aliases the representation's row, unless the module wrote it"]
    info_chirho --> erase_chirho["compile_backend_chirho: erase_newtype_constructors_chirho on those newtypes' constructors before simplification"]
    erase_chirho --> merge_chirho["Native merge: the union of every module's erased set, refusing a spelling another module declares as a constructor"]
    alias_chirho --> erase_chirho
    erase_chirho --> backends_chirho["Interpreter, LLVM and Cranelift all receive the field itself"]
```

## Rules

- An explicit `newtype` never falls back to a stock instance. On a data type it derives
  nothing and declaration validity reports GHC-10333.
- A written instance with a context on a newtype head (`instance Show Int => Show Wrap`) is
  not GND: only the newtype's own deriving declarations put a class in
  `newtype_derived_classes_chirho`, so the dictionary pass never replaces its methods.
- A GND head drops the trailing variables the class's last parameter needs, read from the
  builtin class scope naming proved: `Functor`, `Applicative` and `Monad` drop one. A local or
  imported source class keeps the full head, as before.
- The native backends box an ordinary constructor as a tag word plus its fields. Only the
  constructors of newtypes with a newtype-routed class are erased from Core; every other
  newtype keeps its native representation.

- Root's fresh GHC 9.14.1 contracts (A1-A3, B1-B4, C1-C5, D1-D2, O1-O4) pin the policy: a
  written strategy needs DerivingStrategies or DerivingVia; anyclass needs DeriveAnyClass; an
  unwritten non-stock class needs DeriveAnyClass, or GND on a newtype; DeriveAnyClass wins over
  GND (warning GHC-20042); the last edition decides, and a written flag beats any edition.
- Each module erases its own GND-represented constructors; the native merge erases the union in
  every module, and refuses by name a spelling another module also declares as a constructor.

## Residuals

- A standalone `deriving via` keeps its existing route.
- Classes over a higher-kinded parameter that naming does not prove keep the full GND head.
- GHC2024's additions other than DerivingStrategies belong to the D03 lane.
