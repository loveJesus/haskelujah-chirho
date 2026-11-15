// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn long_output_uses_machine_continuations_without_host_recursion_chirho() {
    let text_chirho = "x".repeat(50_000);
    let source_chirho = format!("main = putStrLn \"{text_chirho}\"\n");
    let outcome_chirho = run_main_chirho(&source_chirho, Default::default()).unwrap();
    outcome_chirho.result_chirho.unwrap();
    assert_eq!(
        outcome_chirho.machine_chirho.io_output_chirho,
        text_chirho + "\n"
    );
}

fn fixture_chirho(group_chirho: &str, name_chirho: &str) -> String {
    std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/frontend_chirho/next_chirho/fixtures_chirho/io_chirho")
            .join(group_chirho)
            .join(format!("{name_chirho}.hs")),
    )
    .unwrap()
}

#[test]
fn source_print_defaulting_unicode_and_lazy_unused_actions_execute_chirho() {
    for (name_chirho, expected_chirho) in [
        ("PrintDefaultChirho", "3\n"),
        ("HelloChirho", "hello\n"),
        ("UnusedActionChirho", "2\n"),
        ("UnicodeOutputChirho", "λ😀\n"),
        ("NegativeIntOutputChirho", "-3\n"),
        (
            "BigIntegerOutputChirho",
            "340282366920938463463374607431768211457\n",
        ),
        ("StoredActionChirho", "stored\n"),
    ] {
        let outcome_chirho = run_main_chirho(
            &fixture_chirho("positive-chirho", name_chirho),
            Default::default(),
        )
        .unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho}"));
        outcome_chirho
            .result_chirho
            .unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho}"));
        assert_eq!(
            outcome_chirho.machine_chirho.io_output_chirho.as_bytes(),
            expected_chirho.as_bytes(),
            "{name_chirho}"
        );
    }
}

#[test]
fn chunk_boundaries_preserve_committed_output_and_discard_the_spare_chirho() {
    for length_chirho in [2046, 2047, 2048] {
        let outcome_chirho = run_main_chirho(
            &fixture_chirho("positive-chirho", &format!("Chunk{length_chirho}Chirho")),
            Default::default(),
        )
        .unwrap();
        outcome_chirho.result_chirho.unwrap();
        assert_eq!(
            outcome_chirho.machine_chirho.io_output_chirho,
            "a".repeat(length_chirho) + "\n"
        );
    }
    for (name_chirho, committed_chirho) in [
        ("BottomPrintedChirho", 0),
        ("ShortBottomTailChirho", 0),
        ("BottomActionChirho", 0),
        ("FullBottomTailChirho", 0),
        ("FullBottomHeadChirho", 2047),
        ("AlmostBottomHeadChirho", 0),
        ("NextBottomTailChirho", 2047),
    ] {
        let outcome_chirho = run_main_chirho(
            &fixture_chirho("failure-chirho", name_chirho),
            Default::default(),
        )
        .unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho}"));
        let error_chirho = outcome_chirho
            .result_chirho
            .expect_err("a demanded incomplete case must fail");
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::ExecutionChirho,
            "{name_chirho}: {error_chirho}"
        );
        assert!(
            error_chirho
                .message_chirho
                .contains("no matching alternative"),
            "{name_chirho}: {error_chirho}"
        );
        assert_eq!(
            outcome_chirho.machine_chirho.io_output_chirho,
            "a".repeat(committed_chirho),
            "{name_chirho}"
        );
    }
}

#[test]
fn output_bytes_and_execution_steps_are_finite_latched_resources_chirho() {
    let source_chirho = fixture_chirho("positive-chirho", "UnicodeOutputChirho");
    for (limit_chirho, success_chirho) in [(6, false), (7, true)] {
        let outcome_chirho = run_main_chirho(
            &source_chirho,
            RunLimitsChirho {
                output_bytes_chirho: limit_chirho,
                ..Default::default()
            },
        )
        .unwrap();
        if success_chirho {
            outcome_chirho.result_chirho.unwrap();
            assert_eq!(outcome_chirho.machine_chirho.io_output_chirho, "λ😀\n");
        } else {
            assert_eq!(
                outcome_chirho.result_chirho.unwrap_err().kind_chirho,
                NextErrorKindChirho::ResourceChirho
            );
            assert!(
                outcome_chirho
                    .machine_chirho
                    .next_io_chirho
                    .exhausted_chirho()
            );
            assert!(outcome_chirho.machine_chirho.io_output_chirho.is_empty());
        }
    }
    let source_chirho = fixture_chirho("positive-chirho", "Chunk2048Chirho");
    let outcome_chirho = run_main_chirho(
        &source_chirho,
        RunLimitsChirho {
            output_bytes_chirho: 2047,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(
        outcome_chirho.result_chirho.unwrap_err().kind_chirho,
        NextErrorKindChirho::ResourceChirho
    );
    assert_eq!(
        outcome_chirho.machine_chirho.io_output_chirho,
        "a".repeat(2047)
    );
    let outcome_chirho = run_main_chirho(
        &source_chirho,
        RunLimitsChirho {
            steps_chirho: 1,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(
        outcome_chirho.result_chirho.unwrap_err().kind_chirho,
        NextErrorKindChirho::ResourceChirho
    );
    assert!(
        run_main_chirho(
            &source_chirho,
            RunLimitsChirho {
                steps_chirho: 0,
                ..Default::default()
            }
        )
        .is_err()
    );
}

#[test]
fn main_requires_the_exported_defining_io_and_library_absence_stays_partial_chirho() {
    let producer_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule AbstractProducerChirho (BoxChirho, makeChirho) where\ndata BoxChirho = HiddenChirho\nmakeChirho = HiddenChirho\n";
    let facade_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule AbstractFacadeChirho (BoxChirho, makeChirho) where\nimport AbstractProducerChirho (BoxChirho, makeChirho)\n";
    let use_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule AbstractUseChirho where\nimport AbstractFacadeChirho (BoxChirho, makeChirho)\nvalueChirho :: BoxChirho\nvalueChirho = makeChirho\n";
    check_sources_chirho(
        &[producer_chirho, facade_chirho, use_chirho],
        "AbstractUseChirho",
    )
    .unwrap();
    let hidden_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule AbstractUseChirho where\nimport AbstractFacadeChirho (BoxChirho, makeChirho)\nvalueChirho = HiddenChirho\n";
    assert_eq!(
        check_sources_chirho(
            &[producer_chirho, facade_chirho, hidden_chirho],
            "AbstractUseChirho"
        )
        .unwrap_err()
        .kind_chirho,
        NextErrorKindChirho::InvalidChirho
    );
    check_source_chirho("module VisibleIOChirho where\nimport GHC.Internal.Types (IO)\nactionChirho :: IO ()\nactionChirho = putStrLn \"ok\"\n").unwrap();
    for source_chirho in [
        "module Main where\nimport GHC.Internal.Types (IO)\nmain :: IO ()\nmain = putStrLn \"ok\"\n",
        "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Main where\nimport GHC.Internal.Types (IO)\nimport GHC.Internal.System.IO (putStrLn)\nmain :: IO ()\nmain = putStrLn \"ok\"\n",
    ] {
        let outcome_chirho = run_main_chirho(source_chirho, Default::default()).unwrap();
        outcome_chirho.result_chirho.unwrap();
        assert_eq!(outcome_chirho.machine_chirho.io_output_chirho, "ok\n");
    }
    for source_chirho in [
        "module Main where\ndata IO aChirho = MkIOChirho\nmain :: IO ()\nmain = putStrLn \"wrong\"\n",
        "module Main where\nimport Prelude (print)\nmain :: IO ()\nmain = print 5\n",
    ] {
        assert_eq!(
            check_source_chirho(source_chirho).unwrap_err().kind_chirho,
            NextErrorKindChirho::InvalidChirho,
            "{source_chirho}"
        );
    }
    let error_chirho = check_source_chirho("module WithheldChirho where\nimport GHC.Internal.IO.Handle.Types\nresultChirho = FileHandle\n").unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    for name_chirho in ["MainIntChirho", "MissingMainChirho", "HomeIOChirho"] {
        let error_chirho =
            check_source_chirho(&fixture_chirho("negative-chirho", name_chirho)).unwrap_err();
        assert!(
            matches!(
                error_chirho.kind_chirho,
                NextErrorKindChirho::InvalidChirho | NextErrorKindChirho::TypeChirho
            ),
            "{name_chirho}: {error_chirho}"
        );
    }
    for name_chirho in ["AbsentBoolShowChirho", "WithheldHandleConstructorsChirho"] {
        let error_chirho =
            check_source_chirho(&fixture_chirho("boundary-chirho", name_chirho)).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{name_chirho}: {error_chirho}"
        );
    }
    let checked_chirho =
        check_source_chirho("main = putStrLn \"ok\"\nhelperChirho = True\n").unwrap();
    assert_eq!(checked_chirho.module_chirho.exports_chirho.len(), 1);
    for source_chirho in [
        "module Main (helperChirho) where\nmain = putStrLn \"ok\"\nhelperChirho = True\n",
        "module OtherChirho where\nmain = putStrLn \"ok\"\n",
    ] {
        let error_chirho = match run_main_chirho(source_chirho, Default::default()) {
            Err(error_chirho) => error_chirho,
            Ok(_) => panic!("run must require Main's exported main"),
        };
        assert_eq!(error_chirho.kind_chirho, NextErrorKindChirho::InvalidChirho);
    }
}

#[test]
fn completion_forces_the_action_but_never_its_lifted_result_chirho() {
    let mut core_chirho = compile_source_chirho("main = putStrLn \"unused\"\n").unwrap();
    let ids_chirho =
        super::super::source_interfaces_chirho::entry_ids_chirho(&core_chirho).unwrap();
    let binder_chirho = |id_chirho, ty_chirho| BinderChirho {
        id_chirho,
        name_chirho: format!("probe{}Chirho", id_chirho.0),
        ty_chirho,
        span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
    };
    let unit_chirho = CoreTypeChirho::TupleChirho(Vec::new());
    let bottom_chirho = CoreExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(CoreExprChirho::ConAppChirho {
            con_name_chirho: "()".into(),
            args_chirho: Vec::new(),
        }),
        bind_chirho: binder_chirho(ids_chirho[1], unit_chirho.clone()),
        result_ty_chirho: unit_chirho.clone(),
        alts_chirho: Vec::new(),
    };
    let main_chirho = core_chirho
        .bindings_chirho
        .iter_mut()
        .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == "main")
        .unwrap();
    main_chirho.rhs_chirho = CoreExprChirho::LamChirho {
        binder_chirho: binder_chirho(ids_chirho[0], world_type_chirho()),
        body_chirho: Box::new(CoreExprChirho::ConAppChirho {
            con_name_chirho: RESULT_KEY_CHIRHO.into(),
            args_chirho: vec![bottom_chirho.clone()],
        }),
    };
    let outcome_chirho = run_core_chirho(core_chirho.clone(), Default::default()).unwrap();
    outcome_chirho
        .result_chirho
        .expect("forcing completion must not force its bottom payload");
    assert!(outcome_chirho.machine_chirho.io_output_chirho.is_empty());
    let main_chirho = core_chirho
        .bindings_chirho
        .iter_mut()
        .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == "main")
        .unwrap();
    main_chirho.rhs_chirho = CoreExprChirho::LamChirho {
        binder_chirho: binder_chirho(ids_chirho[0], world_type_chirho()),
        body_chirho: Box::new(bottom_chirho),
    };
    let outcome_chirho = run_core_chirho(core_chirho, Default::default()).unwrap();
    assert_eq!(
        outcome_chirho.result_chirho.unwrap_err().kind_chirho,
        NextErrorKindChirho::ExecutionChirho
    );
}

#[test]
fn scalar_escape_validation_never_substitutes_surrogate_source_text_chirho() {
    for source_chirho in [
        r#"main = putStrLn "\xD800""#,
        r#"main = putStrLn ['\xD800']"#,
    ] {
        assert_eq!(
            check_source_chirho(source_chirho).unwrap_err().kind_chirho,
            NextErrorKindChirho::UnsupportedChirho
        );
    }
    let outcome_chirho =
        run_main_chirho(r#"main = putStrLn "\x3BB\x1F600""#, Default::default()).unwrap();
    outcome_chirho.result_chirho.unwrap();
    assert_eq!(outcome_chirho.machine_chirho.io_output_chirho, "λ😀\n");
}
