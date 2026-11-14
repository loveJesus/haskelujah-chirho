// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use crate::next_chirho::{
    NextErrorKindChirho, check_source_chirho, compile_source_chirho, eval_binding_chirho,
};
use haskelujah_runtime_chirho::ValueChirho;

fn fixture_chirho(group_chirho: &str, name_chirho: &str) -> String {
    std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/frontend_chirho/next_chirho/fixtures_chirho/int_chirho")
            .join(group_chirho)
            .join(format!("{name_chirho}.hs")),
    )
    .unwrap()
}
fn int_chirho(source_chirho: &str, entry_chirho: &str, expected_chirho: i64) {
    let (value_chirho, mut machine_chirho) =
        eval_binding_chirho(source_chirho, entry_chirho, 200_000)
            .unwrap_or_else(|error_chirho| panic!("{entry_chirho}: {error_chirho}"));
    assert_eq!(
        machine_chirho.force_to_prim_chirho(value_chirho),
        ValueChirho::IntChirho(expected_chirho),
        "{entry_chirho}"
    );
}
fn char_chirho(source_chirho: &str, entry_chirho: &str, expected_chirho: char) {
    let (value_chirho, _) = eval_binding_chirho(source_chirho, entry_chirho, 200_000)
        .unwrap_or_else(|error_chirho| panic!("{entry_chirho}: {error_chirho}"));
    assert_eq!(
        value_chirho,
        ValueChirho::CharChirho(expected_chirho),
        "{entry_chirho}"
    );
}

#[test]
fn int_arithmetic_conversion_and_source_abs_signum_chirho() {
    let source_chirho = fixture_chirho("positive-chirho", "IntArithmeticChirho");
    for (entry_chirho, expected_chirho) in [
        ("nestedChirho", 9),
        ("wrappedAddChirho", i64::MIN),
        ("wrappedNegateChirho", i64::MIN),
        ("wrappedMultiplyChirho", 0),
        ("wrappedSubtractChirho", i64::MAX),
        ("convertedLowChirho", 5),
        ("convertedNegativeChirho", i64::MAX),
        ("minimumAbsChirho", i64::MIN),
        ("negativeAbsChirho", 7),
        ("negativeSignumChirho", -1),
        ("zeroSignumChirho", 0),
        ("positiveSignumChirho", 1),
        ("convertedTypedChirho", 5),
    ] {
        int_chirho(&source_chirho, entry_chirho, expected_chirho);
    }
}

#[test]
fn boxed_tuple_limit_applies_to_types_expressions_and_patterns_chirho() {
    for arity_chirho in [64, 65] {
        let types_chirho = vec!["Bool"; arity_chirho].join(",");
        let values_chirho = vec!["True"; arity_chirho].join(",");
        let patterns_chirho = vec!["_"; arity_chirho].join(",");
        for declaration_chirho in [
            format!("data ManyChirho = ManyChirho ({types_chirho})"),
            format!("valueChirho = ({values_chirho})"),
            format!("functionChirho ({patterns_chirho}) = True"),
        ] {
            let result_chirho = check_source_chirho(&format!(
                "module TupleLimitChirho where\n{declaration_chirho}\n"
            ));
            if arity_chirho == 64 {
                result_chirho.unwrap();
            } else {
                assert_eq!(
                    result_chirho.unwrap_err().kind_chirho,
                    NextErrorKindChirho::InvalidChirho
                );
            }
        }
    }
}

#[test]
fn source_int_instances_and_wired_constructor_results_chirho() {
    let source_chirho = fixture_chirho("positive-chirho", "IntComparisonsChirho");
    for entry_chirho in [
        "equalChirho",
        "unequalChirho",
        "lessChirho",
        "lessEqualChirho",
        "greaterChirho",
        "greaterEqualChirho",
    ] {
        char_chirho(&source_chirho, entry_chirho, 't');
    }
    char_chirho(&source_chirho, "orderChirho", 'l');
    char_chirho(&source_chirho, "notChirho", 'f');
    int_chirho(&source_chirho, "maximumChirho", 3);
    int_chirho(&source_chirho, "minimumChirho", 2);
}

#[test]
fn integer_and_home_names_do_not_become_int_bodies_chirho() {
    let source_chirho = fixture_chirho("positive-chirho", "IntIdentityChirho");
    int_chirho(&source_chirho, "resultChirho", 5);
    char_chirho(&source_chirho, "compareChirho", 't');
    let (value_chirho, _) = eval_binding_chirho(&source_chirho, "hugeChirho", 200_000).unwrap();
    let ValueChirho::IntegerChirho(value_chirho) = value_chirho else {
        panic!("defaulting must retain Integer")
    };
    assert_eq!(value_chirho.to_string(), "18446744073709551617");
    char_chirho(
        &fixture_chirho("positive-chirho", "HomeIntChirho"),
        "resultChirho",
        'h',
    );
}

#[test]
fn boolean_guard_order_and_partial_applications_stay_lazy_chirho() {
    let source_chirho = fixture_chirho("positive-chirho", "GuardOrderChirho");
    for (entry_chirho, expected_chirho) in [
        ("resultChirho", 'p'),
        ("negativeChirho", 'n'),
        ("firstTrueChirho", 'f'),
    ] {
        char_chirho(&source_chirho, entry_chirho, expected_chirho);
    }
    int_chirho(
        &fixture_chirho("positive-chirho", "IntPartialChirho"),
        "resultChirho",
        7,
    );
}

#[test]
fn demanded_operands_and_unmatched_guards_fail_at_execution_chirho() {
    for (file_chirho, entries_chirho) in [
        (
            "IntStrictChirho",
            &["leftChirho", "rightChirho", "negateChirho", "convertChirho"][..],
        ),
        ("GuardFailureChirho", &["resultChirho"][..]),
    ] {
        let source_chirho = fixture_chirho("failure-chirho", file_chirho);
        check_source_chirho(&source_chirho).unwrap();
        for entry_chirho in entries_chirho {
            let Err(error_chirho) = eval_binding_chirho(&source_chirho, entry_chirho, 200_000)
            else {
                panic!("demand must fail");
            };
            assert_eq!(
                error_chirho.kind_chirho,
                NextErrorKindChirho::ExecutionChirho
            );
            assert!(
                error_chirho
                    .message_chirho
                    .contains("no matching alternative"),
                "{entry_chirho}: {error_chirho}"
            );
        }
    }
}

#[test]
fn unsupported_guards_and_hidden_layout_do_not_acquire_authority_chirho() {
    for name_chirho in [
        "MultipleGuardEquationsChirho",
        "CompoundGuardsChirho",
        "HiddenLayoutChirho",
    ] {
        let error_chirho =
            check_source_chirho(&fixture_chirho("boundary-chirho", name_chirho)).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{error_chirho}"
        );
    }
    assert_eq!(
        check_source_chirho(&fixture_chirho("boundary-chirho", "OtherwiseClashChirho"))
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::InvalidChirho
    );
}

#[test]
fn declared_bodies_are_illegal_outside_their_instance_method_root_chirho() {
    let mut checked_chirho =
        check_source_chirho("module OrdinaryChirho where\nidentityChirho xChirho = xChirho\n")
            .unwrap();
    let owner_chirho = checked_chirho.module_chirho.bindings_chirho[0].name_chirho;
    checked_chirho.module_chirho.bindings_chirho[0]
        .body_chirho
        .kind_chirho = haskelujah_hir_chirho::ExprKindChirho::DeclaredBodyChirho(owner_chirho);
    assert_eq!(
        crate::next_chirho::core_chirho::elaborate_chirho(&checked_chirho)
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    assert_eq!(
        haskelujah_tc_chirho::check_module_chirho(checked_chirho.module_chirho, 1 << 20)
            .unwrap_err()
            .kind_chirho,
        haskelujah_tc_chirho::TcErrorKindChirho::UnsupportedChirho
    );
}

#[test]
fn int_only_core_refuses_every_unsupported_native_bridge_chirho() {
    use haskelujah_core_chirho::expr_chirho::{CoreExprChirho, CoreLitChirho};
    let mut core_chirho = compile_source_chirho(
        "module IntOnlyChirho where\nresultChirho :: Int\nresultChirho = 3\n",
    )
    .unwrap();
    let mut binding_chirho = core_chirho
        .bindings_chirho
        .iter()
        .find(|binding_chirho| {
            core_chirho
                .names_chirho
                .get(&binding_chirho.binder_chirho.id_chirho)
                .is_some_and(|name_chirho| name_chirho == "resultChirho")
        })
        .unwrap()
        .clone();
    binding_chirho.rhs_chirho = CoreExprChirho::PrimOpChirho {
        name_chirho: "%next:int-add-chirho".into(),
        args_chirho: vec![
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
        ],
    };
    core_chirho.bindings_chirho = vec![binding_chirho];
    assert!(
        !core_chirho.uses_primitive_chirho(
            haskelujah_core_chirho::expr_chirho::INTEGER_LITERAL_PRIMOP_CHIRHO
        )
    );
    assert!(core_chirho.requires_opaque_integer_runtime_chirho());
    for result_chirho in [
        haskelujah_backend_llvm_chirho::try_compile_core_to_llvm_chirho(&core_chirho).map(|_| ()),
        haskelujah_backend_llvm_chirho::try_compile_core_to_llvm_executable_chirho(&core_chirho)
            .map(|_| ()),
        haskelujah_backend_wasm_chirho::try_compile_core_to_wasm_chirho(&core_chirho).map(|_| ()),
        haskelujah_backend_wasm_chirho::try_compile_core_to_wasm_executable_chirho(&core_chirho)
            .map(|_| ()),
        haskelujah_backend_cranelift_chirho::compile_core_to_object_chirho(
            &core_chirho,
            &Default::default(),
        )
        .map(|_| ()),
        haskelujah_backend_cranelift_chirho::compile_core_to_object_executable_chirho(
            &core_chirho,
            &Default::default(),
        )
        .map(|_| ()),
        haskelujah_backend_cranelift_chirho::jit_chirho::jit_execute_chirho(&core_chirho)
            .map(|_| ()),
    ] {
        assert!(result_chirho.unwrap_err().contains("Integer"));
    }
}
