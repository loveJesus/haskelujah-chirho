// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 692, C10, design B (gpt #32647): a compiler class whose spelling the program's own class
//! takes keeps its own dictionaries. Each program here is in the GHC reference plan
//! `c10-controls`; the interpreter, LLVM and Cranelift must each print GHC's output. A displaced
//! request this pass cannot serve is refused by name and never answered by the program's own
//! instance (gpt #32666).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_driver::{
    FrontendOptionsChirho, compile_source_chirho, eval_source_outcome_with_options_chirho,
};
use haskelujah_span_chirho::SourceMapChirho;

use super::assert_prints_everywhere_chirho;

/// No namesake: an instance of the compiler's own Num keeps its literal and arithmetic rows.
const USER_COMPILER_NUM_INSTANCE_CHIRHO: &str =
    include_str!("user_compiler_num_instance_chirho.hs");
/// The program's own `fromInteger`, a compiler literal and `Prelude.fromInteger` in one module.
const SOURCE_COMPILER_COEXIST_CHIRHO: &str = include_str!("source_compiler_coexist_chirho.hs");
/// The Prelude's `sum` is constrained by the compiler's Num whatever the program spells.
const PRELUDE_SUM_UNDER_NAMESAKE_CHIRHO: &str =
    include_str!("prelude_sum_under_namesake_chirho.hs");
/// A literal in a constructor field, the shape the selector form keeps for every backend.
const BOX_UNDER_NAMESAKE_CHIRHO: &str = include_str!("box_under_namesake_chirho.hs");
/// The compiler's `Show [Maybe Int]` while the program's own `Show` takes the spelling.
const SHOW_LIST_UNDER_NAMESAKE_CHIRHO: &str = include_str!("show_list_under_namesake_chirho.hs");
/// A compiler-qualified polymorphic binding: its own dictionary serves `+` and a literal.
const OWN_DICTIONARY_METHOD_UNDER_NAMESAKE_CHIRHO: &str =
    include_str!("own_dictionary_method_under_namesake_chirho.hs");
/// A constraint at a ground type, whose dictionary the definition decides.
const GROUND_CONSTRAINED_DEFINITION_UNDER_NAMESAKE_CHIRHO: &str =
    include_str!("ground_constrained_definition_under_namesake_chirho.hs");
/// The early equality route: the Prelude's `elem` at Double under a program's own `Eq`.
const ELEM_UNDER_NAMESAKE_EQ_CHIRHO: &str = include_str!("elem_under_namesake_eq_chirho.hs");
/// The early power route: the Prelude's `(^^)` under a program's own `Fractional`.
const POWER_UNDER_NAMESAKE_FRACTIONAL_CHIRHO: &str =
    include_str!("power_under_namesake_fractional_chirho.hs");
/// The early extremum route: the Prelude's `maximum` under a program's own `Ord`.
const MAXIMUM_UNDER_NAMESAKE_ORD_CHIRHO: &str =
    include_str!("maximum_under_namesake_ord_chirho.hs");
/// A reference to a compiler-constrained function at the binding's own dictionary.
const POLYMORPHIC_REFERENCE_UNDER_NAMESAKE_CHIRHO: &str =
    include_str!("polymorphic_reference_under_namesake_chirho.hs");
/// An infix use of a compiler-constrained function, served with no reference evidence.
const INFIX_CONSTRAINED_UNDER_NAMESAKE_CHIRHO: &str =
    include_str!("infix_constrained_under_namesake_chirho.hs");
/// The Prelude's `fromIntegral`, a helper this pass writes over the compiler's Num.
const FROM_INTEGRAL_UNDER_NAMESAKE_CHIRHO: &str =
    include_str!("from_integral_under_namesake_chirho.hs");
/// The Prelude's `sum` and `product` at the binding's own compiler Num dictionary.
const OWN_SCOPE_SUM_PRODUCT_UNDER_NAMESAKE_CHIRHO: &str =
    include_str!("../displaced_own_scope_chirho/own_scope_sum_product_under_namesake_chirho.hs");
/// The Prelude's `print` at the binding's own compiler Show dictionary.
const OWN_SCOPE_PRINT_UNDER_NAMESAKE_SHOW_CHIRHO: &str =
    include_str!("../displaced_own_scope_chirho/own_scope_print_under_namesake_show_chirho.hs");
/// The compiler's undisplaced `Real` reaches the compiler's `Num` through its superclass slot.
const OWN_SCOPE_REAL_SUPERCLASS_UNDER_NAMESAKE_CHIRHO: &str = include_str!(
    "../displaced_own_scope_chirho/own_scope_real_superclass_under_namesake_chirho.hs"
);
/// A source class whose superclass is the compiler's `Typeable` at a transformed head.
const TRANSFORMED_SUPERCLASS_HEAD_UNDER_NAMESAKE_CHIRHO: &str = include_str!(
    "../displaced_own_scope_chirho/transformed_superclass_head_under_namesake_chirho.hs"
);
/// A use that demands the compiler's `Ord [Double]` dictionary itself under a program's own `Eq`.
const DEMANDED_LIST_ORDER_UNDER_NAMESAKE_EQ_CHIRHO: &str =
    include_str!("../displaced_demand_chirho/demanded_list_order_under_namesake_eq_chirho.hs");
/// The same dictionary, demanded only through a program class's Ord superclass field.
const TRANSITIVELY_DEMANDED_LIST_ORDER_UNDER_NAMESAKE_EQ_CHIRHO: &str = include_str!(
    "../displaced_demand_chirho/transitively_demanded_list_order_under_namesake_eq_chirho.hs"
);

#[test]
fn a_user_instance_of_the_compiler_num_keeps_its_own_body_chirho() {
    assert_prints_everywhere_chirho(
        "user_compiler_num_instance_chirho.hs",
        USER_COMPILER_NUM_INSTANCE_CHIRHO,
        "500m\n",
    );
}

#[test]
fn the_programs_own_num_and_the_compilers_num_coexist_chirho() {
    assert_prints_everywhere_chirho(
        "source_compiler_coexist_chirho.hs",
        SOURCE_COMPILER_COEXIST_CHIRHO,
        "8\n7\n7\n",
    );
}

#[test]
fn the_prelude_sum_keeps_the_compilers_num_under_a_namesake_chirho() {
    assert_prints_everywhere_chirho(
        "prelude_sum_under_namesake_chirho.hs",
        PRELUDE_SUM_UNDER_NAMESAKE_CHIRHO,
        "6\n2\n",
    );
}

#[test]
fn a_literal_in_a_constructor_field_keeps_the_compilers_num_under_a_namesake_chirho() {
    assert_prints_everywhere_chirho(
        "box_under_namesake_chirho.hs",
        BOX_UNDER_NAMESAKE_CHIRHO,
        "42\n",
    );
}

/// GHC prints `[Just 1]`. This pass builds no compiler dictionary for `Show [Maybe Int]` while
/// the program's own `Show` takes the spelling, so the module is refused by name; the program's
/// own instance never answers the compiler's request.
#[test]
fn a_displaced_compiler_show_it_cannot_build_is_refused_by_name_chirho() {
    let message_chirho = match compile_source_chirho(
        SHOW_LIST_UNDER_NAMESAKE_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "show_list_under_namesake_chirho.hs",
    ) {
        Ok(_) => panic!("served, where it must be refused by name"),
        Err(error_chirho) => error_chirho.to_string(),
    };
    assert!(
        message_chirho.contains("the compiler's `Show` method")
            && message_chirho.contains("while the program's own class takes the spelling `Show`"),
        "refused, but not by the displaced compiler request: {message_chirho}"
    );
}

/// gpt #32675, route 2: a compiler method at the binding's own dictionary keeps its class proof,
/// so `+` and the literal use the compiler's selectors on that dictionary.
#[test]
fn a_compiler_own_dictionary_method_keeps_its_proof_under_a_namesake_chirho() {
    assert_prints_everywhere_chirho(
        "own_dictionary_method_under_namesake_chirho.hs",
        OWN_DICTIONARY_METHOD_UNDER_NAMESAKE_CHIRHO,
        "42\n2.5\n",
    );
}

/// gpt #32675, route 3: the definition decides a ground compiler predicate's dictionary in the
/// compiler namespace, and its callers read the same decision.
#[test]
fn a_ground_constrained_definition_decides_in_the_compiler_namespace_chirho() {
    assert_prints_everywhere_chirho(
        "ground_constrained_definition_under_namesake_chirho.hs",
        GROUND_CONSTRAINED_DEFINITION_UNDER_NAMESAKE_CHIRHO,
        "42\n",
    );
}

/// gpt #32675, route 1, equality: `elem` compares through the compiler's `==` at Double.
#[test]
fn the_early_equality_route_keeps_the_compilers_eq_under_a_namesake_chirho() {
    assert_prints_everywhere_chirho(
        "elem_under_namesake_eq_chirho.hs",
        ELEM_UNDER_NAMESAKE_EQ_CHIRHO,
        "True\n",
    );
}

/// gpt #32675, route 1, power: `(^^)` divides through the compiler's `/` at Double.
#[test]
fn the_early_power_route_keeps_the_compilers_fractional_under_a_namesake_chirho() {
    assert_prints_everywhere_chirho(
        "power_under_namesake_fractional_chirho.hs",
        POWER_UNDER_NAMESAKE_FRACTIONAL_CHIRHO,
        "0.5\n",
    );
}

/// gpt #32675, route 1, extremum. GHC prints `3`. The compiler namespace holds no default `max`
/// row (the default helpers read spelled donor rows), so `maximum` is served from the
/// compiler's own dictionary, whose `max` fails by name. It never orders through the program's
/// own `compare`, which would print `1`. When the namespace gains GHC's defaults, this control
/// becomes `assert_prints_everywhere_chirho` with GHC's output.
#[test]
fn a_displaced_compiler_maximum_without_a_namespace_default_fails_by_name_chirho() {
    let outcome_chirho = eval_source_outcome_with_options_chirho(
        MAXIMUM_UNDER_NAMESAKE_ORD_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "maximum_under_namesake_ord_chirho.hs",
        None,
        &FrontendOptionsChirho::for_program_chirho("main"),
    )
    .and_then(|outcome_chirho| outcome_chirho.into_result_chirho());
    match outcome_chirho {
        Ok((_, machine_chirho)) => panic!(
            "served `{}`: the named failure no longer holds, so compare with GHC's `3`",
            machine_chirho.io_output_chirho
        ),
        Err(error_chirho) => assert!(
            error_chirho
                .to_string()
                .contains("missing method Ord.max for Int"),
            "failed, but not by the compiler's own named row: {error_chirho}"
        ),
    }
}

/// gpt #32681: a constrained reference at the binding's own dictionary takes that dictionary
/// through its carried class proof. `doubleChirho`'s compiler selectors only work on the
/// compiler's dictionary, so passing the source's would not print these.
#[test]
fn a_polymorphic_reference_passes_its_own_compiler_dictionary_chirho() {
    assert_prints_everywhere_chirho(
        "polymorphic_reference_under_namesake_chirho.hs",
        POLYMORPHIC_REFERENCE_UNDER_NAMESAKE_CHIRHO,
        "20\n6.0\n",
    );
}

/// gpt #32683: with no reference evidence (an infix use), the callee's slots name its compiler
/// namespace, so the dictionary chosen by the call's key is the compiler's.
#[test]
fn an_infix_constrained_call_takes_its_slots_namespace_chirho() {
    assert_prints_everywhere_chirho(
        "infix_constrained_under_namesake_chirho.hs",
        INFIX_CONSTRAINED_UNDER_NAMESAKE_CHIRHO,
        "42\n",
    );
}

/// gpt #32683, a compatibility gap: GHC prints `3.0`. The pass writes `fromIntegral` with
/// selectors named by spelling, which the program's own `Num` owns here, so its references are
/// refused by name instead of answering through the program's class.
#[test]
fn a_compiler_helper_over_a_displaced_class_is_refused_by_name_chirho() {
    let message_chirho = match compile_source_chirho(
        FROM_INTEGRAL_UNDER_NAMESAKE_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "from_integral_under_namesake_chirho.hs",
    ) {
        Ok(_) => panic!("served, where it must be refused by name"),
        Err(error_chirho) => error_chirho.to_string(),
    };
    assert!(
        message_chirho.contains("the compiler-owned helper `fromIntegral`")
            && message_chirho.contains("which the program's own class takes here"),
        "refused, but not by the displaced compiler helper: {message_chirho}"
    );
}

/// gpt #32692: an own-key aggregate takes the own-scope route with the binding's indexed
/// dictionary; the ground route, which has no scope, never refuses it first.
#[test]
fn an_own_scope_sum_and_product_keep_the_compilers_num_chirho() {
    assert_prints_everywhere_chirho(
        "own_scope_sum_product_under_namesake_chirho.hs",
        OWN_SCOPE_SUM_PRODUCT_UNDER_NAMESAKE_CHIRHO,
        "12\n6.5\n",
    );
}

/// gpt #32692: an own-key `print` shows through the compiler's own `show` selector on the
/// binding's indexed dictionary; the method-name selector is the source class's.
#[test]
fn an_own_scope_print_keeps_the_compilers_show_chirho() {
    assert_prints_everywhere_chirho(
        "own_scope_print_under_namesake_show_chirho.hs",
        OWN_SCOPE_PRINT_UNDER_NAMESAKE_SHOW_CHIRHO,
        "7\n[True,False]\n",
    );
}

/// gpt #32919: the compiler's `Real`, though no program class displaces it, keeps the compiler's
/// `Num` as its superclass slot, so `foldr (Prelude.+) 0` under `Real a` sums through the
/// compiler's `+` and literal, never the program's own `Num`, whose Int instance adds one.
#[test]
fn an_undisplaced_compiler_real_keeps_the_compilers_num_superclass_chirho() {
    assert_prints_everywhere_chirho(
        "own_scope_real_superclass_under_namesake_chirho.hs",
        OWN_SCOPE_REAL_SUPERCLASS_UNDER_NAMESAKE_CHIRHO,
        "6\n",
    );
}

/// gpt #32942, the reachable boundary: a source class whose superclass is the compiler's
/// `Typeable` at `Maybe a` while the program's own `Typeable` takes the spelling. No route may
/// serve that superclass at the child's key `Int`, so the module is refused by name. GHC accepts
/// the program and prints `weird`: this is a named gap, not parity.
#[test]
fn a_compiler_superclass_at_a_transformed_head_is_refused_by_name_chirho() {
    let message_chirho = match compile_source_chirho(
        TRANSFORMED_SUPERCLASS_HEAD_UNDER_NAMESAKE_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "transformed_superclass_head_under_namesake_chirho.hs",
    ) {
        Ok(_) => panic!("served, where it must be refused by name"),
        Err(error_chirho) => error_chirho.to_string(),
    };
    assert!(
        message_chirho.contains("the compiler's `Typeable` superclass at a transformed head")
            && message_chirho
                .contains("while the program's own class takes the spelling `Typeable`"),
        "refused, but not by the transformed-head refusal: {message_chirho}"
    );
}

/// gpt #33201, a demanded use: `sameChirho` at `[Double]` passes the compiler's `Ord [Double]`
/// dictionary, whose Eq superclass slot this pass cannot build while the program's own `Eq` takes
/// the spelling. The module is refused by that dictionary's key alone; the speculative list
/// dictionaries no use reaches refuse nothing. GHC accepts the program and prints `True`: this is
/// a named gap, not parity.
#[test]
fn a_demanded_compiler_ord_list_is_refused_by_its_own_key_chirho() {
    assert_refused_at_the_demanded_key_chirho(
        "demanded_list_order_under_namesake_eq_chirho.hs",
        DEMANDED_LIST_ORDER_UNDER_NAMESAKE_EQ_CHIRHO,
    );
}

/// gpt #33201, a transitive demand: the use passes only the program's `RankChirho [Double]`
/// dictionary, whose Ord superclass field is the compiler's `Ord [Double]`. GHC prints `2`: this is
/// a named gap, not parity.
#[test]
fn a_transitively_demanded_compiler_ord_list_is_refused_by_its_own_key_chirho() {
    assert_refused_at_the_demanded_key_chirho(
        "transitively_demanded_list_order_under_namesake_eq_chirho.hs",
        TRANSITIVELY_DEMANDED_LIST_ORDER_UNDER_NAMESAKE_EQ_CHIRHO,
    );
}

/// The module is refused by the compiler's Eq superclass slot at the demanded key `[Double]`, and
/// at no list key that no use demands.
fn assert_refused_at_the_demanded_key_chirho(file_name_chirho: &str, source_chirho: &str) {
    let message_chirho = match compile_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
    ) {
        Ok(_) => panic!("{file_name_chirho}: served, where it must be refused by name"),
        Err(error_chirho) => error_chirho.to_string(),
    };
    assert!(
        message_chirho.contains("the compiler's `Eq` superclass slot at `[Double]`")
            && message_chirho.contains("while the program's own class takes the spelling `Eq`"),
        "{file_name_chirho}: refused, but not by the demanded dictionary: {message_chirho}"
    );
    for unused_key_chirho in ["`[Int]`", "`[Bool]`", "`[[Char]]`"] {
        assert!(
            !message_chirho.contains(unused_key_chirho),
            "{file_name_chirho}: a list dictionary no use demands refused the module: {message_chirho}"
        );
    }
}
