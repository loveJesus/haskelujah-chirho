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
    cst_chirho --> lower_chirho["Lowering deriving_clauses_chirho.rs: each entry through the flat type lowering, one DerivedClassChirho per class with its written arguments and its clause's DerivingStrategyChirho; via clauses stay in deriving_via_chirho"]
    standalone_chirho["CST and lowering: deriving [strategy] instance; StandaloneDerivingDeclChirho.strategy_chirho; the head's last type is the derived one, the types before it the class's arguments"]
    pragmas_chirho["Parser module_extensions_chirho: each written flag after every edition; GHC2024 adds DerivingStrategies"] --> policy_chirho["extension_policy_chirho.rs DerivingExtensionsChirho"]
    policy_chirho --> validity_chirho
    policy_chirho --> route_chirho
    scope_chirho --> identity_chirho["stock_identity_chirho.rs StockIdentityChirho: K01's proof names the builtin stock class; a spelling the module declares itself is never one; any other unproven spelling keeps its written name"]
    identity_chirho --> validity_chirho
    identity_chirho --> route_chirho
    lower_chirho --> validity_chirho["Declaration validity deriving_strategy_chirho.rs, rules 7-15: GHC-87139, 10333, 26557, 38178, 00158, 82023, 73993 (non-unary class application), 00158 (stock on a non-stock class), Unsupported (newtype strategy with class arguments)"]
    standalone_chirho --> validity_chirho
    lower_chirho --> route_chirho["strategy_route_chirho.rs DerivingPolicyChirho::route_chirho: written stock, anyclass or newtype; an unwritten strategy keeps the legacy policy"]
    standalone_chirho --> route_chirho
    scope_chirho["Pipeline: build_builtin_class_scope_chirho before deriving (K01)"] --> route_chirho
    route_chirho -->|stock| stock_chirho["declaration_heads_chirho.rs derive_stock_class_chirho: the proven class's generator; spell_generated_class_chirho names the instance's class as written"]
    route_chirho -->|anyclass| empty_chirho["derive_anyclass_chirho: an empty instance over the full head C args (T vs), using the class defaults"]
    route_chirho -->|newtype| gnd_chirho["newtype_route_chirho.rs derive_newtype_instance_chirho: C rep => C N, eta-reduced by the builtin class's parameter arity, no methods"]
    route_chirho --> inventory_chirho["newtype_strategy_classes_chirho: each newtype's newtype-routed classes"]
    stock_chirho --> metadata_chirho["Show and Enum metadata: only entries the same judgment names as stock Show or Enum"]
    inventory_chirho --> info_chirho["Driver build_newtype_info_chirho, with the module's real K01 scope: NewtypeInfoChirho.newtype_derived_classes_chirho"]
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
- A class keeps its written arguments. The anyclass route derives `C args (T vs)` through the
  ordinary multi-parameter instance path. The stock generators and the GND dictionary write one
  head type, so declaration validity refuses a class applied to arguments there: GHC-73993 for a
  stock class, which takes none, and Unsupported for the newtype strategy. GHC accepts the
  latter (W3 prints `MeterChirho 12`), so it is a compatibility gap, not GHC coverage.
- Stock deriving runs the generator of the builtin class K01 proved for the written spelling,
  and names the instance's class as written, so `deriving stock (P.Eq)` under a qualified
  Prelude import is the Prelude's `Eq`. `deriving stock (Eq)` beside a local `class Eq` is
  GHC-00158. K01 excludes a source re-export as well as a local declaration, so only a local
  declaration refuses; Read and Generic, which K01's table does not hold, keep their written
  name.
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
- The newtype strategy for a class applied to arguments (W3) waits for a dictionary key that
  names every head type.
- Typing still keys classes by unqualified name, so a module with a local `class Eq` and the
  Prelude's `Eq` in one program is not yet told apart past deriving (S2's end-to-end run).
- Generated method bodies name their helpers unqualified, so a module that hides or qualifies the
  Prelude's `==`, `&&` or `showString` gives them its own binding.
- Classes over a higher-kinded parameter that naming does not prove keep the full GND head.
- GHC2024's additions other than DerivingStrategies belong to the D03 lane.
