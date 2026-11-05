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
    pragmas_chirho["Parser module_extensions_chirho: the last edition's flags, then each written flag; GHC2024 adds DerivingStrategies and RoleAnnotations"] --> policy_chirho["extension_policy_chirho.rs DerivingExtensionsChirho"]
    policy_chirho --> validity_chirho
    policy_chirho --> route_chirho
    scope_chirho --> identity_chirho["stock_identity_chirho.rs StockIdentityChirho: an explicit stock takes a generator only on K01's proof; an unwritten strategy keeps the legacy route, which takes a written name K01 cannot judge; a spelling the module declares itself is never a builtin stock class"]
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
    roles_annotation_chirho["Lowering role_annotations_chirho.rs: type role T r.. onto T's parameters"] --> roles_chirho["parameter_roles_chirho.rs: each local parameter's role (phantom < representational < Unknown < nominal), a least fixed point with annotations as floors"]
    prefix_lower_chirho["Lowering con_prefixes_chirho.rs: an H98 constructor's forall binders and context onto ConPrefixChirho, or unread"] --> roles_chirho
    inventory_chirho --> proof_chirho["Typing gnd_coercion_chirho (after inference): every method's class parameter in a representational position, or GHC-18872 / a named Unsupported gap"]
    roles_chirho --> proof_chirho
    proof_chirho --> info_chirho
    info_chirho --> eligible_chirho["Dict pass newtype_dicts_chirho.rs takes_representation_dictionary_chirho: the declaration is the only proof"]
    gnd_chirho --> eligible_chirho
    eligible_chirho --> ground_skip_chirho["generate_instance_dicts_chirho skips it"]
    eligible_chirho --> alias_chirho["generate_gnd_dicts_chirho, a worklist: with typing's proof, each method is the representation's real row, or its dictionary's field, building a conditional [E] dictionary once E's context exists; a desugar default row is replaced in place behind its leading context dictionaries; the dictionary's fields are the methods; a stub, a written row or an unbuildable dictionary is Unsupported"]
    alias_chirho --> refuse_chirho["compile_backend: DictPassResultChirho.unsupported_chirho refuses the module"]
    eligible_chirho --> parametric_chirho["generate_parametric_gnd_factories_chirho: C (N a) gets a factory over its context dictionaries, registered in conditional_dicts_chirho with the head's formal binders"]
    evidence_chirho["Typing applied_evidence_chirho.rs: a reference's evidence and a method occurrence's record carry its ground applied type (BoxChirho Int); the driver's occurrence join keeps it for the record that proves the occurrence"] --> use_chirho
    parametric_chirho --> use_chirho["conditional_dictionary_chirho at a reference, print or a direct method occurrence (method_through_factory_chirho): the factory applied to its context, instantiated at the proved arguments; else refused by name"]
    use_chirho --> refuse_chirho
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
- An explicit `stock` runs the generator of the builtin class K01 proved for the written
  spelling, and names the instance's class as written, so `deriving stock (P.Eq)` under a
  qualified Prelude import is the Prelude's `Eq`. `deriving stock (Eq)` beside a local
  `class Eq`, or a proven builtin class GHC does not derive that way, is GHC-00158. K01
  excludes a source re-export as well as a local declaration, so an exclusion is neither a
  disproof nor an authority: every other unproven spelling (an imported or re-exported source
  class, Read, Generic) is a named Unsupported gap (#31786). An unwritten strategy keeps the
  legacy route, which takes a written name K01 cannot judge.
- The native backends box an ordinary constructor as a tag word plus its fields. Only the
  constructors of newtypes with a newtype-routed class are erased from Core; every other
  newtype keeps its native representation.

- A newtype takes its representation's method rows only with a typed proof, per (newtype,
  class), that every method's class parameter stands where a coercion may change it: directly,
  under `->`, a list or a tuple, as an eta-reduced head, or under a local type's
  representational parameter. A local nominal parameter or a local type family's argument is
  GHC-18872 (R1, R2 and the H98 equality context H1, measured); below a blocked position every
  occurrence is blocked, phantom positions included, as GHC's `markNominal` does. A position
  whose role the module cannot prove (an imported, qualified or synonym constructor, a GADT's
  parameter, an unread constructor prefix, a type variable's application, a constraint), an
  associated family (R5) or a multi-parameter class is a named Unsupported gap, never GHC's
  code. A method that never mentions the parameter proves (R4).
- Role authority is the local declaration a name binds: an unqualified name, or one qualified
  by this module's own name (#31810). A constructor's existential binders are not parameters,
  and every variable its context mentions is nominal (#31811). A prefix the lowering cannot
  read (a kinded binder, a context outside plain type syntax, a variable the lowering would
  lose) and a GADT record's signature are unread: every parameter is at least Unknown.
- A GND instance is `C rep => C N`, so a desugar default row of it takes that context's
  dictionaries first, as every caller passes them; its replacement keeps those leading
  parameters (G3, D2, C1, C2).
- A GND instance over a parametric newtype (`Show a => Show (BoxChirho a)`, P1, P2) is a
  factory: a lambda over its context dictionaries whose dictionary reads every method from the
  context dictionary of its own class, the representation's. Each superclass field is the
  newtype's own `S (N a)` dictionary, found by its instance record (#31851, S1): a written
  instance without a context is the dictionary registered under the newtype, and one the
  newtype also takes through the newtype strategy is that class's factory at the
  representation's `S` dictionary. A written instance with a context, several instances or
  none refuse the factory by name, and a use that needs it is refused when the module
  compiles. The representation's own superclass dictionary is never copied. It is the conditional instance's
  builder (`conditional_dicts_chirho`), with the head's formal binders. A reference whose
  evidence carries a ground applied type (`BoxChirho Int`, never parsed from a key) gets the
  factory applied to the context instantiated at those arguments: the registry's dictionary,
  or another factory's. `print` takes `show` from that dictionary instead of the generic
  `showInt#` fallback, and a direct method occurrence the checker proved at a ground
  application (`describeChirho (BoxChirho True)`, M1) takes its selector over that dictionary
  before any row: the instance's own rows wait for a context. A use the factory cannot serve
  is refused when the module compiles.
- The alias loop requires typing's proof and never aliases a generated missing-method stub.
  A representation's method is its real row, or its dictionary's field; for a list
  representation whose class instance is conditional, the existing list machinery builds the
  `[E]` dictionary once `E`'s context dictionaries exist, which may be another GND newtype's
  (`[Age]`), so a worklist, not pass or table order, decides. The desugarer names the rows it
  fills from class defaults, and only those are replaced; anything unbuildable is a compile
  error. The list machinery finds a list instance's rows under the variable its head is
  written with (`[aChirho]`).
- One builtin class K01 proves has at most one instance for a type's whole head, however each
  spells it (`P.Show` and `Show`): GHC-59692, measured.
- Root's fresh GHC 9.14.1 contracts (A1-A3, B1-B4, C1-C5, D1-D2, O1-O4) pin the policy: a
  written strategy needs DerivingStrategies or DerivingVia; anyclass needs DeriveAnyClass; an
  unwritten non-stock class needs DeriveAnyClass, or GND on a newtype; DeriveAnyClass wins over
  GND (warning GHC-20042); the last edition decides, and a written flag beats any edition.
- Each module erases its own GND-represented constructors; the native merge erases the union in
  every module, and refuses by name a spelling another module also declares as a constructor.

## Residuals

- A parametric GND dictionary at a non-ground application (a rigid variable's) is not served
  yet: a reference that needs one is refused at compile time, and a direct method occurrence
  at one (`describeChirho` inside `f :: DescribeChirho a => BoxChirho a -> String`) keeps the
  head-keyed route, the separately named non-ground limit.
- An H98 context written without `forall` (`data T a = Eq a => MkT a`) is not parsed as a
  context; GADT signatures and roles of imported types are Unknown, not read.

- A standalone `deriving via` keeps its existing route.
- The newtype strategy for a class applied to arguments (W3) waits for a dictionary key that
  names every head type.
- Typing still keys classes by unqualified name, so a module with a local `class Eq` and the
  Prelude's `Eq` in one program is not yet told apart past deriving (S2's end-to-end run).
- Generated method bodies name their helpers unqualified, so a module that hides or qualifies the
  Prelude's `==`, `&&` or `showString` gives them its own binding.
- Classes over a higher-kinded parameter that naming does not prove keep the full GND head.
- GHC2024's additions other than DerivingStrategies belong to the D03 lane.
