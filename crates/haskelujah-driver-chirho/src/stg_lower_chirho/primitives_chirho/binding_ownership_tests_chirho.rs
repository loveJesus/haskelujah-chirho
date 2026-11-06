// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A value the program owns keeps its own binding whatever its spelling (#32355). Each of the
//! lowering's four reference paths asks the one IO facade, which lets a spelling reach only an id
//! the program does not own. The four paths are a bare reference, the head of an application, an
//! argument, and a constructor field. A free reference and a binding the compiler generated keep
//! the builtin primitive, and an unlinked import fails closed by name. The last control follows a
//! real import from its producer, the module's provenance, to both interpreter consumers (#32361).
//! workflow: testing-chirho/execution-oracles-chirho

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use haskelujah_core_chirho::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, CoreProvenanceChirho, ImportedReferenceChirho,
    InlineAnnotationChirho,
};
use haskelujah_runtime_chirho::ValueChirho;
use haskelujah_span_chirho::{SourceMapChirho, SpanChirho};
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use crate::stg_lower_chirho::lower_and_run_outcome_chirho;

const MAIN_CHIRHO: u32 = 0;
/// The id spelled like an interpreter primitive in each control.
const SPELLED_CHIRHO: u32 = 1;
const APPLY_CHIRHO: u32 = 2;
/// A binding the compiler generated, in a module that records its own; nothing references it.
const GENERATED_CHIRHO: u32 = 9;

/// The provenance of a module that records what the compiler generated, as the dictionary pass's
/// output does. A top-level binding outside that record is then the program's own.
fn recorded_chirho() -> CoreProvenanceChirho {
    CoreProvenanceChirho {
        compiler_bindings_chirho: BTreeSet::from([CoreIdChirho(GENERATED_CHIRHO)]),
        ..CoreProvenanceChirho::default()
    }
}

fn binder_chirho(name_chirho: &str, id_chirho: u32) -> BinderChirho {
    BinderChirho {
        id_chirho: CoreIdChirho(id_chirho),
        name_chirho: name_chirho.to_string(),
        ty_chirho: TyChirho::int_chirho(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn var_chirho(id_chirho: u32) -> CoreExprChirho {
    CoreExprChirho::VarChirho(CoreIdChirho(id_chirho))
}

fn int_chirho(value_chirho: i64) -> CoreExprChirho {
    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(value_chirho))
}

fn apply_chirho(function_chirho: CoreExprChirho, arguments_chirho: &[i64]) -> CoreExprChirho {
    apply_exprs_chirho(
        function_chirho,
        arguments_chirho
            .iter()
            .map(|value_chirho| int_chirho(*value_chirho))
            .collect(),
    )
}

fn apply_exprs_chirho(
    function_chirho: CoreExprChirho,
    arguments_chirho: Vec<CoreExprChirho>,
) -> CoreExprChirho {
    arguments_chirho
        .into_iter()
        .fold(function_chirho, |fun_chirho, arg_chirho| {
            CoreExprChirho::AppChirho {
                fun_chirho: Box::new(fun_chirho),
                arg_chirho: Box::new(arg_chirho),
            }
        })
}

fn lambda_chirho(parameters_chirho: &[(&str, u32)], body_chirho: CoreExprChirho) -> CoreExprChirho {
    parameters_chirho
        .iter()
        .rev()
        .fold(body_chirho, |body_chirho, (name_chirho, id_chirho)| {
            CoreExprChirho::LamChirho {
                binder_chirho: binder_chirho(name_chirho, *id_chirho),
                body_chirho: Box::new(body_chirho),
            }
        })
}

fn let_chirho(
    name_chirho: &str,
    id_chirho: u32,
    rhs_chirho: CoreExprChirho,
    body_chirho: CoreExprChirho,
) -> CoreExprChirho {
    CoreExprChirho::LetChirho {
        rec_chirho: false,
        binds_chirho: vec![(binder_chirho(name_chirho, id_chirho), rhs_chirho)],
        body_chirho: Box::new(body_chirho),
    }
}

fn binding_chirho(
    name_chirho: &str,
    id_chirho: u32,
    rhs_chirho: CoreExprChirho,
) -> CoreBindingChirho {
    CoreBindingChirho {
        binder_chirho: binder_chirho(name_chirho, id_chirho),
        rhs_chirho,
        is_rec_chirho: false,
        inline_chirho: InlineAnnotationChirho::NoneChirho,
    }
}

/// The program's own two-argument value, whose answer is never the primitive's: 99.
fn own_value_chirho() -> CoreExprChirho {
    lambda_chirho(&[("a", 20), ("b", 21)], int_chirho(99))
}

fn module_chirho(
    bindings_chirho: Vec<CoreBindingChirho>,
    spelling_chirho: &str,
    provenance_chirho: CoreProvenanceChirho,
) -> CoreModuleChirho {
    CoreModuleChirho {
        name_chirho: "Main".to_string(),
        bindings_chirho,
        names_chirho: HashMap::from([
            (CoreIdChirho(MAIN_CHIRHO), "main".to_string()),
            (CoreIdChirho(SPELLED_CHIRHO), spelling_chirho.to_string()),
            (CoreIdChirho(APPLY_CHIRHO), "applyChirho".to_string()),
        ]),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        provenance_chirho,
    }
}

fn run_main_chirho(module_chirho: &CoreModuleChirho) -> Result<ValueChirho, String> {
    lower_and_run_outcome_chirho(module_chirho, Some("main"), HashSet::new())
        .and_then(|outcome_chirho| outcome_chirho.result_chirho)
}

#[test]
fn an_applied_spelling_reaches_only_a_value_the_program_does_not_own_chirho() {
    // `seq 1 7`: the primitive answers 7, the program's own `seq` answers 99.
    let main_chirho = binding_chirho(
        "main",
        MAIN_CHIRHO,
        apply_chirho(var_chirho(SPELLED_CHIRHO), &[1, 7]),
    );
    let own_seq_chirho = binding_chirho("seq", SPELLED_CHIRHO, own_value_chirho());
    let generated_chirho = CoreProvenanceChirho {
        compiler_bindings_chirho: BTreeSet::from([CoreIdChirho(SPELLED_CHIRHO)]),
        ..CoreProvenanceChirho::default()
    };
    let cases_chirho = [
        (
            "a top-level binding the program owns",
            vec![own_seq_chirho.clone(), main_chirho.clone()],
            recorded_chirho(),
            99,
        ),
        (
            "a module without a record of its generated bindings keeps the legacy top-level route",
            vec![own_seq_chirho.clone(), main_chirho.clone()],
            CoreProvenanceChirho::default(),
            7,
        ),
        (
            "a binding the compiler generated keeps the primitive",
            vec![own_seq_chirho, main_chirho.clone()],
            generated_chirho,
            7,
        ),
        (
            "a free reference keeps the primitive",
            vec![main_chirho],
            CoreProvenanceChirho::default(),
            7,
        ),
        (
            "a let-bound value",
            vec![binding_chirho(
                "main",
                MAIN_CHIRHO,
                let_chirho(
                    "seq",
                    SPELLED_CHIRHO,
                    own_value_chirho(),
                    apply_chirho(var_chirho(SPELLED_CHIRHO), &[1, 7]),
                ),
            )],
            CoreProvenanceChirho::default(),
            99,
        ),
    ];
    for (reason_chirho, bindings_chirho, provenance_chirho, expected_chirho) in cases_chirho {
        assert_eq!(
            run_main_chirho(&module_chirho(bindings_chirho, "seq", provenance_chirho)),
            Ok(ValueChirho::IntChirho(expected_chirho)),
            "{reason_chirho}"
        );
    }
}

#[test]
fn a_bare_reference_keeps_a_bound_value_spelled_undefined_chirho() {
    // A where-bound and a top-level `undefined` are 7. A free `undefined` is still the primitive,
    // so entering it fails.
    let local_chirho = binding_chirho(
        "main",
        MAIN_CHIRHO,
        let_chirho(
            "undefined",
            SPELLED_CHIRHO,
            int_chirho(7),
            var_chirho(SPELLED_CHIRHO),
        ),
    );
    let top_level_chirho = vec![
        binding_chirho("undefined", SPELLED_CHIRHO, int_chirho(7)),
        binding_chirho("main", MAIN_CHIRHO, var_chirho(SPELLED_CHIRHO)),
    ];
    for (reason_chirho, bindings_chirho, provenance_chirho) in [
        (
            "where-bound",
            vec![local_chirho],
            CoreProvenanceChirho::default(),
        ),
        ("top-level", top_level_chirho, recorded_chirho()),
    ] {
        assert_eq!(
            run_main_chirho(&module_chirho(
                bindings_chirho,
                "undefined",
                provenance_chirho
            )),
            Ok(ValueChirho::IntChirho(7)),
            "{reason_chirho}"
        );
    }
    assert!(
        run_main_chirho(&module_chirho(
            vec![binding_chirho(
                "main",
                MAIN_CHIRHO,
                var_chirho(SPELLED_CHIRHO)
            )],
            "undefined",
            CoreProvenanceChirho::default()
        ))
        .is_err()
    );
}

#[test]
fn a_bound_value_passed_as_an_argument_keeps_its_binding_chirho() {
    // `applyChirho f = f 1 7`, given the program's top-level and then a let-bound `seq`.
    let apply_binding_chirho = binding_chirho(
        "applyChirho",
        APPLY_CHIRHO,
        lambda_chirho(&[("f", 30)], apply_chirho(var_chirho(30), &[1, 7])),
    );
    let passes_seq_chirho =
        || apply_exprs_chirho(var_chirho(APPLY_CHIRHO), vec![var_chirho(SPELLED_CHIRHO)]);
    let top_level_chirho = vec![
        binding_chirho("seq", SPELLED_CHIRHO, own_value_chirho()),
        apply_binding_chirho.clone(),
        binding_chirho("main", MAIN_CHIRHO, passes_seq_chirho()),
    ];
    let let_bound_chirho = vec![
        apply_binding_chirho,
        binding_chirho(
            "main",
            MAIN_CHIRHO,
            let_chirho(
                "seq",
                SPELLED_CHIRHO,
                own_value_chirho(),
                passes_seq_chirho(),
            ),
        ),
    ];
    for (reason_chirho, bindings_chirho, provenance_chirho) in [
        ("top-level", top_level_chirho, recorded_chirho()),
        (
            "let-bound",
            let_bound_chirho,
            CoreProvenanceChirho::default(),
        ),
    ] {
        assert_eq!(
            run_main_chirho(&module_chirho(bindings_chirho, "seq", provenance_chirho)),
            Ok(ValueChirho::IntChirho(99)),
            "{reason_chirho}"
        );
    }
}

#[test]
fn a_bound_value_in_a_constructor_field_keeps_its_binding_chirho() {
    // `case BoxChirho seq of BoxChirho f -> f 2 3`, with the program's own `seq`.
    let main_chirho = binding_chirho(
        "main",
        MAIN_CHIRHO,
        CoreExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(CoreExprChirho::ConAppChirho {
                con_name_chirho: "BoxChirho".to_string(),
                args_chirho: vec![var_chirho(SPELLED_CHIRHO)],
            }),
            bind_chirho: binder_chirho("box", 40),
            result_ty_chirho: TyChirho::int_chirho(),
            alts_chirho: vec![CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho("BoxChirho".to_string()),
                binders_chirho: vec![binder_chirho("f", 41)],
                rhs_chirho: apply_chirho(var_chirho(41), &[2, 3]),
            }],
        },
    );
    assert_eq!(
        run_main_chirho(&module_chirho(
            vec![
                binding_chirho("seq", SPELLED_CHIRHO, own_value_chirho()),
                main_chirho
            ],
            "seq",
            recorded_chirho()
        )),
        Ok(ValueChirho::IntChirho(99))
    );
}

#[test]
fn case_and_lambda_binders_keep_their_values_chirho() {
    // `case 5 of seq { DEFAULT -> seq }` and `(\seq -> seq) 6`.
    let case_chirho = CoreExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(int_chirho(5)),
        bind_chirho: binder_chirho("seq", SPELLED_CHIRHO),
        result_ty_chirho: TyChirho::int_chirho(),
        alts_chirho: vec![CoreAltChirho {
            con_chirho: AltConChirho::DefaultChirho,
            binders_chirho: vec![],
            rhs_chirho: var_chirho(SPELLED_CHIRHO),
        }],
    };
    let applied_lambda_chirho = apply_chirho(
        lambda_chirho(&[("seq", SPELLED_CHIRHO)], var_chirho(SPELLED_CHIRHO)),
        &[6],
    );
    for (reason_chirho, main_rhs_chirho, expected_chirho) in [
        ("case binder", case_chirho, 5),
        ("lambda parameter", applied_lambda_chirho, 6),
    ] {
        assert_eq!(
            run_main_chirho(&module_chirho(
                vec![binding_chirho("main", MAIN_CHIRHO, main_rhs_chirho)],
                "seq",
                CoreProvenanceChirho::default()
            )),
            Ok(ValueChirho::IntChirho(expected_chirho)),
            "{reason_chirho}"
        );
    }
}

#[test]
fn an_unlinked_import_fails_closed_by_its_name_chirho() {
    // Another program module's `seq`, which this Core does not link: never the primitive.
    let imported_chirho = CoreProvenanceChirho {
        imported_references_chirho: BTreeMap::from([(
            CoreIdChirho(SPELLED_CHIRHO),
            ImportedReferenceChirho {
                name_chirho: "seq".to_string(),
                defining_modules_chirho: vec!["OwnSeqChirho".to_string()],
            },
        )]),
        ..CoreProvenanceChirho::default()
    };
    let error_chirho = run_main_chirho(&module_chirho(
        vec![binding_chirho(
            "main",
            MAIN_CHIRHO,
            apply_chirho(var_chirho(SPELLED_CHIRHO), &[1, 7]),
        )],
        "seq",
        imported_chirho,
    ))
    .expect_err("an unlinked import fails closed");
    assert!(
        error_chirho.contains("missing STG binding `seq`"),
        "{error_chirho}"
    );
}

/// A program module that exports its own `putStrLn`.
const OWN_PUT_MODULE_CHIRHO: &str = "module OwnPutChirho (putStrLn) where\n\nimport Prelude hiding (putStrLn)\nimport qualified Prelude as P\n\nputStrLn :: String -> IO ()\nputStrLn s = P.putStrLn (\"own \" ++ s)\n";

/// Main, which uses that export in place of the Prelude's.
const IMPORTING_MAIN_CHIRHO: &str = "module Main where\n\nimport Prelude hiding (putStrLn)\nimport OwnPutChirho (putStrLn)\n\nmain :: IO ()\nmain = putStrLn \"x\"\n";

#[test]
fn a_real_import_spelled_like_io_stays_the_programs_own_unlinked_reference_chirho() {
    // Producer to consumer over the real pipeline. The search-path compile links no sibling
    // Core, so the import is refused by name here. That proves no borrowed Prelude body, not
    // that imports run.
    let directory_chirho = tempfile::tempdir().expect("a scratch directory");
    std::fs::write(
        directory_chirho.path().join("OwnPutChirho.hs"),
        OWN_PUT_MODULE_CHIRHO,
    )
    .expect("the sibling module");
    let compiled_chirho = crate::compile_source_with_search_path_chirho(
        IMPORTING_MAIN_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "Main.hs",
        directory_chirho.path(),
    )
    .unwrap_or_else(|refusal_chirho| panic!("GHC accepts the program: {refusal_chirho}"));
    let core_chirho = &compiled_chirho.core_chirho;
    // The producer: the module's provenance names the reference as the sibling's export.
    let imported_chirho: Vec<_> = core_chirho
        .provenance_chirho
        .imported_references_chirho
        .iter()
        .filter(|(_, reference_chirho)| reference_chirho.name_chirho == "putStrLn")
        .collect();
    let [(imported_id_chirho, reference_chirho)] = imported_chirho.as_slice() else {
        panic!("one imported putStrLn: {:?}", core_chirho.provenance_chirho);
    };
    assert_eq!(
        reference_chirho.defining_modules_chirho,
        vec!["OwnPutChirho".to_string()]
    );
    // The first consumer: the interpreter's preparation manufactures no primitive for it.
    let prepared_chirho =
        haskelujah_core_chirho::io_actions_chirho::prepare_io_actions_chirho(core_chirho, "main");
    assert!(
        prepared_chirho
            .bindings_chirho
            .iter()
            .all(|binding_chirho| binding_chirho.binder_chirho.id_chirho != **imported_id_chirho),
        "a primitive was manufactured for the import"
    );
    // The second consumer: unlinked here, the lowering refuses it by name and prints nothing.
    let outcome_chirho = lower_and_run_outcome_chirho(
        core_chirho,
        Some("main"),
        compiled_chirho.newtype_cons_chirho.clone(),
    )
    .expect("the module lowers");
    let error_chirho = outcome_chirho
        .result_chirho
        .expect_err("an unlinked import fails closed");
    assert!(
        error_chirho.contains("missing STG binding `putStrLn`"),
        "{error_chirho}"
    );
    assert_eq!(outcome_chirho.machine_chirho.io_output_chirho, "");
    // The contrast: the Prelude's own `putStrLn`, a free canonical reference, still runs.
    let canonical_chirho = crate::eval_source_outcome_chirho(
        "module Main where\n\nmain :: IO ()\nmain = putStrLn \"x\"\n",
        &mut SourceMapChirho::new_chirho(),
        "Main.hs",
        Some("main"),
    )
    .expect("the canonical program compiles");
    assert_eq!(canonical_chirho.result_chirho.map(|_| ()), Ok(()));
    assert_eq!(canonical_chirho.machine_chirho.io_output_chirho, "x\n");
}
