// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 694 deriving-strategy controls on every engine.
//!
//! The programs are the fixtures beside this file, shared with the interpreter controls
//! (`src/tests_chirho/extensions_chirho/deriving_strategies_chirho.rs`) and the multi-module
//! controls (`tests/native_symbol_identity_chirho/newtype_representation_chirho.rs`).
//! `newtype_show`, `data_newtype_strategy` and `missing_gnd_extension` are root's D01 controls
//! byte for byte (`26-10-30-chirho/accept-frontier-baseline-chirho.md`), measured with GHC
//! 9.14.1: `7`, GHC-10333 and GHC-26557. Every other expected output is GHC's semantics,
//! still to be measured with GHC 9.14.1. `parametric_newtype_show` is a pinned prediction
//! (#31740): its GND instance has a context, so its dictionary must take that context.
//!
//! The `gnd_methods_chirho` fixtures are the interpreter's direct-method and coercion
//! controls (`deriving_policy_chirho.rs` and `gnd_coercion_chirho.rs`), shared byte for byte:
//! G3, C1 and C2 are root's measured sources (GHC 9.14.1 prints 8, 78, 78), D2 is the
//! alpha-renamed copy of a measured source (prints `int`). Each GND method there is the
//! representation's, reached through a default row that takes its instance's context first
//! (#31826). P1 and P2 are root's measured parametric controls (GHC prints `7`/`True` and
//! `integer`/`boolean`): each argument's dictionary comes from the parametric GND instance's
//! factory (#31829). `parametric_newtype_show` prints through `print`, which reaches that
//! factory only through its `Show` evidence; it passed before any factory existed, by the
//! generic `showInt#` fallback, so it was never dictionary evidence.
//!
//! S2, G1 and G3 are root's measured multi-parameter superclass controls (#31916, #31936):
//! GHC 9.14.1 prints `True`, `AgeChirho 5` and `5`. A1 and W1, measured too, are the interpreter
//! suite's check and compile controls; their output waits on `>>=` at an abstract monad.
//!
//! The `typeable_route_chirho` fixtures are the Typeable route's GHC 9.14.1 references byte
//! for byte (typeable-route r2, r3, r6 and r7, #32090, #32106). A builtin `Typeable` clause,
//! unwritten or with an explicit `stock` or `newtype` strategy, and a standalone `deriving
//! newtype instance`, are ignored, so the stock `Show` beside each prints the constructor
//! (`AgeChirho 5`, `AgeChirho 7`). The module's own
//! `class Typeable` keeps genuine GND (`int`), and without GND it is GHC-82023. Each
//! provider's `import M (Typeable)` is accepted (`typeable`).
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

const NEWTYPE_SHOW_CHIRHO: &str = include_str!("deriving_strategies_chirho/newtype_show_chirho.hs");
const DATA_NEWTYPE_STRATEGY_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/data_newtype_strategy_chirho.hs");
const MISSING_GND_EXTENSION_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/missing_gnd_extension_chirho.hs");
const STOCK_SHOW_CHIRHO: &str = include_str!("deriving_strategies_chirho/stock_show_chirho.hs");
const STANDALONE_NEWTYPE_SHOW_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/standalone_newtype_show_chirho.hs");
const NEWTYPE_NUM_CHIRHO: &str = include_str!("deriving_strategies_chirho/newtype_num_chirho.hs");
const MIXED_STOCK_SHOW_NEWTYPE_NUM_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/mixed_stock_show_newtype_num_chirho.hs");
const NESTED_NEWTYPE_FIELDS_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/nested_newtype_fields_chirho.hs");
const PARAMETRIC_NEWTYPE_SHOW_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/parametric_newtype_show_chirho.hs");
const WRITTEN_PARAMETRIC_SHOW_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/written_parametric_show_chirho.hs");
const CONTEXTUAL_USER_INSTANCE_CHIRHO: &str =
    include_str!("deriving_strategies_chirho/contextual_user_instance_chirho.hs");
const USER_CLASS_NEWTYPE_METHOD_CHIRHO: &str =
    include_str!("gnd_methods_chirho/user_class_newtype_method_chirho.hs");
const NEWTYPE_GND_PRECEDENCE_CHIRHO: &str =
    include_str!("gnd_methods_chirho/newtype_gnd_precedence_chirho.hs");
const CONTEXTUAL_REPRESENTATION_DEFAULT_CHIRHO: &str =
    include_str!("gnd_methods_chirho/contextual_representation_default_chirho.hs");
const NESTED_GND_CONTEXT_DEPENDENCY_CHIRHO: &str =
    include_str!("gnd_methods_chirho/nested_gnd_context_dependency_chirho.hs");
const PARAMETRIC_NO_INLINE_SHOW_CHIRHO: &str =
    include_str!("gnd_methods_chirho/parametric_no_inline_show_chirho.hs");
const PARAMETRIC_USER_DICTIONARY_CHIRHO: &str =
    include_str!("gnd_methods_chirho/parametric_user_dictionary_chirho.hs");
const PARAMETRIC_WRITTEN_SUPERCLASS_CHIRHO: &str =
    include_str!("gnd_methods_chirho/parametric_written_superclass_chirho.hs");
const DIRECT_PARAMETRIC_GND_METHODS_CHIRHO: &str =
    include_str!("gnd_methods_chirho/direct_parametric_gnd_methods_chirho.hs");
const SECOND_ARGUMENT_SUPERCLASS_CHIRHO: &str =
    include_str!("gnd_methods_chirho/second_argument_superclass_chirho.hs");
const STOCK_SUPERCLASS_GROUND_NEWTYPE_CHIRHO: &str =
    include_str!("gnd_methods_chirho/stock_superclass_ground_newtype_chirho.hs");
const NEWTYPE_SUPERCLASS_PARAMETRIC_CHIRHO: &str =
    include_str!("gnd_methods_chirho/newtype_superclass_parametric_chirho.hs");
const TYPEABLE_UNWRITTEN_GND_SHOW_CHIRHO: &str =
    include_str!("typeable_route_chirho/typeable_unwritten_gnd_show_chirho.hs");
const TYPEABLE_UNWRITTEN_NO_GND_CHIRHO: &str =
    include_str!("typeable_route_chirho/typeable_unwritten_no_gnd_chirho.hs");
const TYPEABLE_EXPLICIT_NEWTYPE_SHOW_CHIRHO: &str =
    include_str!("typeable_route_chirho/typeable_explicit_newtype_show_chirho.hs");
const TYPEABLE_EXPLICIT_STOCK_SHOW_CHIRHO: &str =
    include_str!("typeable_route_chirho/typeable_explicit_stock_show_chirho.hs");
const TYPEABLE_STANDALONE_NEWTYPE_SHOW_CHIRHO: &str =
    include_str!("typeable_route_chirho/typeable_standalone_newtype_show_chirho.hs");
const TYPEABLE_LOCAL_GND_CHIRHO: &str =
    include_str!("typeable_route_chirho/typeable_local_gnd_chirho.hs");
const TYPEABLE_LOCAL_NO_GND_PRAGMA_CHIRHO: &str =
    include_str!("typeable_route_chirho/typeable_local_no_gnd_pragma_chirho.hs");
const TYPEABLE_LOCAL_HASKELL2010_CHIRHO: &str =
    include_str!("typeable_route_chirho/typeable_local_haskell2010_chirho.hs");
const TYPEABLE_PROVIDERS_CHIRHO: [(&str, &str); 3] = [
    (
        "TypeableProviderDataTypeableChirho.hs",
        include_str!("typeable_route_chirho/typeable_provider_data_typeable_chirho.hs"),
    ),
    (
        "TypeableProviderTypeReflectionChirho.hs",
        include_str!("typeable_route_chirho/typeable_provider_type_reflection_chirho.hs"),
    ),
    (
        "TypeableProviderDataDataChirho.hs",
        include_str!("typeable_route_chirho/typeable_provider_data_data_chirho.hs"),
    ),
];

#[test]
fn deriving_newtype_show_prints_the_representation_everywhere_chirho() {
    assert_prints_everywhere_chirho("NewtypeShowChirho.hs", NEWTYPE_SHOW_CHIRHO, "7\n");
}

#[test]
fn deriving_stock_show_prints_the_constructor_everywhere_chirho() {
    assert_prints_everywhere_chirho("StockShowChirho.hs", STOCK_SHOW_CHIRHO, "AgeChirho 7\n");
}

#[test]
fn standalone_deriving_newtype_show_prints_the_representation_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "StandaloneNewtypeShowChirho.hs",
        STANDALONE_NEWTYPE_SHOW_CHIRHO,
        "7\n",
    );
}

#[test]
fn deriving_newtype_num_computes_with_the_representation_everywhere_chirho() {
    assert_prints_everywhere_chirho("NewtypeNumChirho.hs", NEWTYPE_NUM_CHIRHO, "7\n");
}

#[test]
fn stock_show_beside_newtype_num_keeps_the_constructor_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "MixedStockShowNewtypeNumChirho.hs",
        MIXED_STOCK_SHOW_NEWTYPE_NUM_CHIRHO,
        "AgeChirho 7\n",
    );
}

#[test]
fn nested_newtype_and_data_fields_agree_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "NestedNewtypeFieldsChirho.hs",
        NESTED_NEWTYPE_FIELDS_CHIRHO,
        "PersonChirho \"Ann\" 7\n8\n6\n",
    );
}

#[test]
fn parametric_deriving_newtype_show_takes_its_context_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "ParametricNewtypeShowChirho.hs",
        PARAMETRIC_NEWTYPE_SHOW_CHIRHO,
        "7\n",
    );
}

#[test]
fn a_written_parametric_show_prints_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "WrittenParametricShowChirho.hs",
        WRITTEN_PARAMETRIC_SHOW_CHIRHO,
        "7\n",
    );
}

#[test]
fn a_contextual_user_instance_keeps_its_own_method_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "ContextualUserInstanceChirho.hs",
        CONTEXTUAL_USER_INSTANCE_CHIRHO,
        "user\n",
    );
}

#[test]
fn a_user_class_method_is_the_representations_everywhere_chirho() {
    // G3: the class default would print "default".
    assert_prints_everywhere_chirho(
        "UserClassNewtypeMethodChirho.hs",
        USER_CLASS_NEWTYPE_METHOD_CHIRHO,
        "8\n",
    );
}

#[test]
fn an_unwritten_user_class_takes_the_representations_method_everywhere_chirho() {
    // D2: the class default would print "described".
    assert_prints_everywhere_chirho(
        "NewtypeGndPrecedenceChirho.hs",
        NEWTYPE_GND_PRECEDENCE_CHIRHO,
        "int\n",
    );
}

#[test]
fn a_contextual_representation_default_is_the_lists_method_everywhere_chirho() {
    // C1: the representation `[Int]`'s instance is conditional.
    assert_prints_everywhere_chirho(
        "ContextualRepresentationDefaultChirho.hs",
        CONTEXTUAL_REPRESENTATION_DEFAULT_CHIRHO,
        "78\n",
    );
}

#[test]
fn a_nested_gnd_context_dependency_resolves_everywhere_chirho() {
    // C2: `[AgeChirho]` needs `AgeChirho`'s GND dictionary first.
    assert_prints_everywhere_chirho(
        "NestedGndContextDependencyChirho.hs",
        NESTED_GND_CONTEXT_DEPENDENCY_CHIRHO,
        "78\n",
    );
}

#[test]
fn a_parametric_gnd_show_dictionary_takes_each_argument_everywhere_chirho() {
    // P1: a NOINLINE `Show a =>` function receives `Show (BoxChirho Int)` and
    // `Show (BoxChirho Bool)` from the instance's factory.
    assert_prints_everywhere_chirho(
        "ParametricNoInlineShowChirho.hs",
        PARAMETRIC_NO_INLINE_SHOW_CHIRHO,
        "7\nTrue\n",
    );
}

#[test]
fn a_parametric_gnd_user_dictionary_takes_each_argument_everywhere_chirho() {
    // P2: the same through a user class with an instance per argument.
    assert_prints_everywhere_chirho(
        "ParametricUserDictionaryChirho.hs",
        PARAMETRIC_USER_DICTIONARY_CHIRHO,
        "integer\nboolean\n",
    );
}

#[test]
fn a_parametric_gnd_dictionary_holds_the_newtypes_own_superclass_everywhere_chirho() {
    // S1 (#31851): GHC 9.14.1 prints `written` (gnd-superclass-r1-chirho, #31854).
    assert_prints_everywhere_chirho(
        "ParametricWrittenSuperclassChirho.hs",
        PARAMETRIC_WRITTEN_SUPERCLASS_CHIRHO,
        "written\n",
    );
}

#[test]
fn direct_parametric_gnd_methods_take_each_argument_everywhere_chirho() {
    // M1 (#31864): GHC 9.14.1 prints these (role-surfaces-and-direct-gnd-r1-chirho, #31865).
    assert_prints_everywhere_chirho(
        "DirectParametricGndMethodsChirho.hs",
        DIRECT_PARAMETRIC_GND_METHODS_CHIRHO,
        "7\nTrue\ninteger\nboolean\n",
    );
}

#[test]
fn a_second_argument_superclass_prints_everywhere_chirho() {
    // S2 (#31909): GHC 9.14.1 prints `True` (mptc-superclass-r1-chirho, #31916).
    assert_prints_everywhere_chirho(
        "SecondArgumentSuperclassChirho.hs",
        SECOND_ARGUMENT_SUPERCLASS_CHIRHO,
        "True\n",
    );
}

#[test]
fn a_ground_class_argument_gnd_holds_its_stock_show_everywhere_chirho() {
    // G1 (#31928): GHC 9.14.1 prints `AgeChirho 5` (mptc-superclass-fallback-r1-chirho, #31936).
    assert_prints_everywhere_chirho(
        "StockSuperclassGroundNewtypeChirho.hs",
        STOCK_SUPERCLASS_GROUND_NEWTYPE_CHIRHO,
        "AgeChirho 5\n",
    );
}

#[test]
fn a_parametric_class_argument_gnd_holds_its_own_show_everywhere_chirho() {
    // G3 (#31928): GHC 9.14.1 prints `5` (mptc-superclass-fallback-r1-chirho, #31936).
    assert_prints_everywhere_chirho(
        "NewtypeSuperclassParametricChirho.hs",
        NEWTYPE_SUPERCLASS_PARAMETRIC_CHIRHO,
        "5\n",
    );
}

#[test]
fn deriving_newtype_on_data_is_refused_chirho() {
    rejected_like_ghc_chirho(
        DATA_NEWTYPE_STRATEGY_CHIRHO,
        "DataNewtypeStrategyChirho.hs",
        &[
            "E0209",
            "`Show BadChirho` with the newtype strategy",
            "cannot be used on non-newtypes",
        ],
    );
}

#[test]
fn deriving_newtype_without_the_extension_is_refused_chirho() {
    rejected_like_ghc_chirho(
        MISSING_GND_EXTENSION_CHIRHO,
        "MissingGndExtensionChirho.hs",
        &[
            "E0209",
            "`Eq AgeChirho` with the newtype strategy",
            "GeneralizedNewtypeDeriving is not enabled",
        ],
    );
}

#[test]
fn a_builtin_typeable_clause_keeps_the_stock_show_constructor_everywhere_chirho() {
    // typeable-route r2, r6 and r7: GHC 9.14.1 ignores the request in each written form.
    for (file_name_chirho, source_chirho, ghc_chirho) in [
        (
            "TypeableUnwrittenGndShowChirho.hs",
            TYPEABLE_UNWRITTEN_GND_SHOW_CHIRHO,
            "AgeChirho 5\n",
        ),
        (
            "TypeableUnwrittenNoGndChirho.hs",
            TYPEABLE_UNWRITTEN_NO_GND_CHIRHO,
            "AgeChirho 7\n",
        ),
        (
            "TypeableExplicitNewtypeShowChirho.hs",
            TYPEABLE_EXPLICIT_NEWTYPE_SHOW_CHIRHO,
            "AgeChirho 5\n",
        ),
        (
            "TypeableExplicitStockShowChirho.hs",
            TYPEABLE_EXPLICIT_STOCK_SHOW_CHIRHO,
            "AgeChirho 5\n",
        ),
        (
            "TypeableStandaloneNewtypeShowChirho.hs",
            TYPEABLE_STANDALONE_NEWTYPE_SHOW_CHIRHO,
            "AgeChirho 5\n",
        ),
    ] {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn a_local_typeable_class_keeps_genuine_gnd_everywhere_chirho() {
    // typeable-route r2: GHC 9.14.1 prints the representation's `int`, never the default.
    assert_prints_everywhere_chirho(
        "TypeableLocalGndChirho.hs",
        TYPEABLE_LOCAL_GND_CHIRHO,
        "int\n",
    );
}

#[test]
fn a_local_typeable_class_without_gnd_is_refused_chirho() {
    // typeable-route r3: GHC-82023 under NoGeneralizedNewtypeDeriving and under Haskell2010.
    for (file_name_chirho, source_chirho) in [
        (
            "TypeableLocalNoGndPragmaChirho.hs",
            TYPEABLE_LOCAL_NO_GND_PRAGMA_CHIRHO,
        ),
        (
            "TypeableLocalHaskell2010Chirho.hs",
            TYPEABLE_LOCAL_HASKELL2010_CHIRHO,
        ),
    ] {
        rejected_like_ghc_chirho(
            source_chirho,
            file_name_chirho,
            &[
                "E0209",
                "`Typeable AgeChirho`",
                "is not a stock derivable class",
            ],
        );
    }
}

#[test]
fn each_typeable_provider_import_runs_on_the_interpreter_chirho() {
    // typeable-route r2: GHC 9.14.1 prints `typeable` for each provider.
    for (file_name_chirho, source_chirho) in TYPEABLE_PROVIDERS_CHIRHO {
        let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
            source_chirho,
            &mut haskelujah_span_chirho::SourceMapChirho::new_chirho(),
            file_name_chirho,
            None,
        )
        .unwrap_or_else(|error_chirho| panic!("{file_name_chirho}: interpreter: {error_chirho}"));
        assert_eq!(
            machine_chirho.io_output_chirho, "typeable\n",
            "{file_name_chirho}"
        );
    }
}
