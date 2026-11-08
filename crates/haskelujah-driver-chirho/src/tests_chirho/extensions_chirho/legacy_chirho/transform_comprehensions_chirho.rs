// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Byte-identical GHC 9.14.1 transform/rank/nesting/scope contracts.
use super::ordinary_elaboration_chirho::frontend_chirho;

macro_rules! accepts_chirho {
    ($test_chirho:ident, $path_chirho:literal) => {
        #[test]
        fn $test_chirho() {
            frontend_chirho(include_str!($path_chirho)).expect("GHC accepts this unchanged source");
        }
    };
}
macro_rules! rejects_chirho {
    ($test_chirho:ident, $path_chirho:literal) => {
        #[test]
        fn $test_chirho() {
            let error_chirho = frontend_chirho(include_str!($path_chirho))
                .err()
                .expect("GHC rejects this unchanged source");
            assert!(error_chirho.contains("error[E"), "{error_chirho}");
        }
    };
}
macro_rules! evaluates_chirho {
    ($test_chirho:ident, $path_chirho:literal, $output_chirho:literal) => {
        #[test]
        fn $test_chirho() {
            let mut sources_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
            let (_, machine_chirho) = crate::eval_source_with_machine_chirho(
                include_str!($path_chirho),
                &mut sources_chirho,
                "TransformChirho.hs",
                None,
            )
            .expect("the real parser/frontend/Core/interpreter path executes");
            assert_eq!(machine_chirho.io_output_chirho, $output_chirho);
        }
    };
}

accepts_chirho!(
    original_mc18_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/typing_chirho/mc18Chirho.hs"
);
accepts_chirho!(
    original_tc240_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/typing_chirho/tc240Chirho.hs"
);
accepts_chirho!(
    parallel_grouping_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/typing_chirho/ParallelGroupingChirho.hs"
);
rejects_chirho!(
    wrong_group_nesting_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/typing_chirho/WrongGroupNestingChirho.hs"
);
rejects_chirho!(
    monomorphic_grouping_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/typing_chirho/MonomorphicGroupingChirho.hs"
);
rejects_chirho!(
    disabled_grouping_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/typing_chirho/GroupingDisabledChirho.hs"
);
rejects_chirho!(
    missing_transformer_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/typing_chirho/MissingTransformerChirho.hs"
);
rejects_chirho!(
    parallel_key_escape_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/typing_chirho/ParallelKeyEscapeChirho.hs"
);

evaluates_chirho!(
    nested_groups_execute_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/runtime_chirho/NestedGroupsExecutableChirho.hs",
    "[[],[[]],[[],[1]],[[],[1],[1,2]]]\n"
);
evaluates_chirho!(
    plain_transform_executes_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/runtime_chirho/PlainTransformExecutableChirho.hs",
    "[3,2,1]\n"
);
evaluates_chirho!(
    grouped_columns_execute_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/runtime_chirho/GroupedColumnsExecutableChirho.hs",
    "[([1,1],[2,3]),([2],[4])]\n"
);
evaluates_chirho!(
    using_expression_keeps_outer_scope_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/runtime_chirho/OuterTransformerExecutableChirho.hs",
    "[[],[1],[1,2]]\n"
);

#[test]
fn flat_repeated_grouping_keeps_the_original_generated_type_bound_chirho() {
    let source_chirho = format!(
        "{{-# LANGUAGE TransformListComp #-}}\n{{-# OPTIONS_GHC -fdefer-type-errors #-}}\nmodule BoundChirho where\nimport Data.List (inits)\nvalueChirho = [ xChirho | xChirho <- ([1] :: [Int]){} ]\n",
        ", then group using inits".repeat(140)
    );
    let error_chirho = frontend_chirho(&source_chirho)
        .err()
        .expect("flat syntax cannot create unbounded nested inferred types");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
    assert!(error_chirho.contains("bound exhausted"), "{error_chirho}");
}

evaluates_chirho!(
    owned_grouped_columns_execute_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/runtime_chirho/OwnedGroupedColumnsChirho.hs",
    "[([1,1],[2,3]),([2],[4])]\n"
);
evaluates_chirho!(
    zero_columns_keep_row_cardinality_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/runtime_chirho/ZeroColumnGuardGroupingChirho.hs",
    "[7,7]\n"
);
rejects_chirho!(
    empty_statement_prefix_is_rejected_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/typing_chirho/ZeroColumnGroupingChirho.hs"
);

evaluates_chirho!(
    parallel_grouping_executes_in_lockstep_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/runtime_chirho/ParallelGroupingExecutableChirho.hs",
    "[([],3),([1],4)]\n"
);
evaluates_chirho!(
    group_keeps_its_ordinary_identifier_alternative_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/runtime_chirho/GroupIdentifierTransformerChirho.hs",
    "[2,1]\n"
);
rejects_chirho!(
    using_is_reserved_under_the_effective_extension_chirho,
    "../kind_repair_fixtures_chirho/syntax_chirho/comprehensions_chirho/typing_chirho/ReservedUsingIdentifierChirho.hs"
);
