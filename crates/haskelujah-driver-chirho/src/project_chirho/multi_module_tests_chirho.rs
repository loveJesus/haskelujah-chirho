// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The interpreter's multi-module merge carries each module's provenance with its ids (#32375):
//! the same offset as the module's bodies, joined with the others, through the same final remap,
//! and with the root's entry alone. The IO facade then tells a merged program's own top-level
//! bindings from the rows the compiler generated, as it does in one module. A builtin reference
//! links only to a compiler body of its name, never to a program namesake (#32386).
//! workflow: testing-chirho/execution-oracles-chirho

use std::collections::BTreeSet;

use haskelujah_core_chirho::expr_chirho::{CoreExprChirho, CoreIdChirho, CoreModuleChirho};
use haskelujah_span_chirho::SourceMapChirho;

use super::{
    compile_modules_chirho, eval_modules_with_machine_chirho, merge_interpreter_modules_chirho,
};
use crate::stg_lower_chirho::lower_and_run_outcome_chirho;

/// A dependency with its own `seq` and `undefined`, and its own `main`: an entry candidate that
/// must never become the program's.
const HELPER_MODULE_CHIRHO: &str = "module HelperChirho (seq, undefined, main) where\n\nimport Prelude hiding (seq, undefined)\n\nseq :: Int -> Int -> Int\nseq a b = a * 10 + b\n\nundefined :: Int\nundefined = 7\n\nmain :: IO ()\nmain = print (seq 1 2)\n";

/// The root, which imports the dependency's values and forces through the generated rows.
const HELPER_MAIN_CHIRHO: &str = "module Main where\n\nimport Prelude hiding (seq, undefined)\nimport Control.DeepSeq (deepseq, force)\nimport HelperChirho (seq, undefined)\n\nmain :: IO ()\nmain = do\n  print (seq 4 2)\n  print (force undefined)\n  undefined `deepseq` putStrLn \"forced\"\n";

/// A dependency whose own values are spelled like interpreter IO primitives.
const OWN_SPELLINGS_MODULE_CHIRHO: &str = "module OwnSpellingsChirho (seq, undefined) where\n\nimport Prelude hiding (seq, undefined)\n\nseq :: Int -> Int -> Int\nseq a b = a * 10 + b\n\nundefined :: Int\nundefined = 7\n";

/// The root: the dependency's own `seq` and `undefined` beside the generated `force` and
/// `deepseq`, whose Core bodies use `seq#`, which the interpreter has no primitive for.
const OWN_SPELLINGS_MAIN_CHIRHO: &str = "module Main where\n\nimport Prelude hiding (seq, undefined)\nimport Control.DeepSeq (deepseq, force)\nimport OwnSpellingsChirho (seq, undefined)\n\nmain :: IO ()\nmain = do\n  print (seq 4 2)\n  print undefined\n  print (force (seq 1 7))\n  undefined `deepseq` putStrLn \"forced\"\n";

/// A dependency with its own `undefined`, which the root never imports.
const UNIMPORTED_UNDEFINED_MODULE_CHIRHO: &str = "module OwnUndefinedChirho (sevenChirho) where\n\nimport Prelude hiding (undefined)\n\nundefined :: Int\nundefined = 7\n\nsevenChirho :: Int\nsevenChirho = undefined\n";

/// The root, whose `undefined` is the Prelude's: GHC prints 7, then fails with
/// `Prelude.undefined`.
const PRELUDE_UNDEFINED_MAIN_CHIRHO: &str = "module Main where\n\nimport OwnUndefinedChirho (sevenChirho)\n\nmain :: IO ()\nmain = do\n  print sevenChirho\n  print (undefined :: Int)\n";

fn moved_chirho(id_chirho: CoreIdChirho, offset_chirho: u32) -> CoreIdChirho {
    CoreIdChirho(id_chirho.0 + offset_chirho)
}

/// Every id an expression references.
fn referenced_ids_chirho(expression_chirho: &CoreExprChirho, out_chirho: &mut BTreeSet<u32>) {
    match expression_chirho {
        CoreExprChirho::VarChirho(id_chirho) => {
            out_chirho.insert(id_chirho.0);
        }
        CoreExprChirho::LitChirho(_) => {}
        CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } => {
            referenced_ids_chirho(fun_chirho, out_chirho);
            referenced_ids_chirho(arg_chirho, out_chirho);
        }
        CoreExprChirho::LamChirho { body_chirho, .. }
        | CoreExprChirho::TyLamChirho { body_chirho, .. } => {
            referenced_ids_chirho(body_chirho, out_chirho);
        }
        CoreExprChirho::LetChirho {
            binds_chirho,
            body_chirho,
            ..
        } => {
            for (_, rhs_chirho) in binds_chirho {
                referenced_ids_chirho(rhs_chirho, out_chirho);
            }
            referenced_ids_chirho(body_chirho, out_chirho);
        }
        CoreExprChirho::CaseChirho {
            scrutinee_chirho,
            alts_chirho,
            ..
        } => {
            referenced_ids_chirho(scrutinee_chirho, out_chirho);
            for alt_chirho in alts_chirho {
                referenced_ids_chirho(&alt_chirho.rhs_chirho, out_chirho);
            }
        }
        CoreExprChirho::TyAppChirho { expr_chirho, .. } => {
            referenced_ids_chirho(expr_chirho, out_chirho);
        }
        CoreExprChirho::PrimOpChirho { args_chirho, .. }
        | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
            for argument_chirho in args_chirho {
                referenced_ids_chirho(argument_chirho, out_chirho);
            }
        }
    }
}

/// Each input module's offset, read from where its bindings landed: the merge appends every
/// module's bindings in order and moves each binder by that module's one offset.
fn landed_offsets_chirho(
    inputs_chirho: &[CoreModuleChirho],
    merged_chirho: &CoreModuleChirho,
) -> Vec<u32> {
    let mut start_chirho = 0;
    inputs_chirho
        .iter()
        .map(|input_chirho| {
            let landed_chirho = &merged_chirho.bindings_chirho[start_chirho..]
                [..input_chirho.bindings_chirho.len()];
            let offset_chirho = landed_chirho[0].binder_chirho.id_chirho.0
                - input_chirho.bindings_chirho[0].binder_chirho.id_chirho.0;
            for (input_binding_chirho, landed_binding_chirho) in
                input_chirho.bindings_chirho.iter().zip(landed_chirho)
            {
                assert_eq!(
                    landed_binding_chirho.binder_chirho.id_chirho,
                    moved_chirho(input_binding_chirho.binder_chirho.id_chirho, offset_chirho),
                    "one offset moves every binder of {}",
                    input_chirho.name_chirho
                );
                assert_eq!(
                    landed_binding_chirho.binder_chirho.name_chirho,
                    input_binding_chirho.binder_chirho.name_chirho
                );
            }
            start_chirho += input_chirho.bindings_chirho.len();
            offset_chirho
        })
        .collect()
}

#[test]
fn the_merge_moves_each_record_with_its_bodies_and_keeps_the_roots_entry_chirho() {
    let results_chirho = compile_modules_chirho(
        &[
            ("HelperChirho.hs", HELPER_MODULE_CHIRHO),
            ("Main.hs", HELPER_MAIN_CHIRHO),
        ],
        &mut SourceMapChirho::new_chirho(),
    )
    .unwrap_or_else(|refusal_chirho| panic!("GHC accepts the program: {refusal_chirho:?}"));
    let inputs_chirho: Vec<CoreModuleChirho> = results_chirho
        .iter()
        .map(|result_chirho| result_chirho.core_chirho.clone())
        .collect();
    let (merged_chirho, _) =
        merge_interpreter_modules_chirho(results_chirho).expect("the modules merge");
    let offsets_chirho = landed_offsets_chirho(&inputs_chirho, &merged_chirho);
    let [helper_chirho, main_chirho] = inputs_chirho.as_slice() else {
        panic!("two modules");
    };
    let record_chirho = &merged_chirho.provenance_chirho;

    // What the compiler generated moves by each module's offset, and nothing joins it.
    let generated_chirho: BTreeSet<CoreIdChirho> = inputs_chirho
        .iter()
        .zip(&offsets_chirho)
        .flat_map(|(input_chirho, offset_chirho)| {
            input_chirho
                .provenance_chirho
                .compiler_bindings_chirho
                .iter()
                .map(move |id_chirho| moved_chirho(*id_chirho, *offset_chirho))
        })
        .collect();
    assert!(!generated_chirho.is_empty());
    assert_eq!(record_chirho.compiler_bindings_chirho, generated_chirho);
    assert!(
        merged_chirho.bindings_chirho.iter().any(|binding_chirho| {
            binding_chirho.binder_chirho.name_chirho == "force"
                && generated_chirho.contains(&binding_chirho.binder_chirho.id_chirho)
        }),
        "a generated force row is recorded"
    );
    for own_chirho in ["seq", "undefined"] {
        let binding_chirho = merged_chirho
            .bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == own_chirho)
            .unwrap_or_else(|| panic!("the dependency's own {own_chirho}"));
        assert!(
            !generated_chirho.contains(&binding_chirho.binder_chirho.id_chirho),
            "the dependency's own {own_chirho} is the program's"
        );
    }

    // The root alone supplies the entry, though the dependency has an entry candidate too.
    let root_entry_chirho = main_chirho
        .provenance_chirho
        .entry_chirho
        .expect("the root's main");
    let helper_entry_chirho = helper_chirho
        .provenance_chirho
        .entry_chirho
        .expect("the dependency's own main");
    assert_eq!(
        record_chirho.entry_chirho,
        Some(moved_chirho(root_entry_chirho, offsets_chirho[1]))
    );
    assert_ne!(
        record_chirho.entry_chirho,
        Some(moved_chirho(helper_entry_chirho, offsets_chirho[0]))
    );

    // An import follows the final remap to the definition it names, as the bodies do.
    let mut referenced_chirho = BTreeSet::new();
    for binding_chirho in &merged_chirho.bindings_chirho {
        referenced_ids_chirho(&binding_chirho.rhs_chirho, &mut referenced_chirho);
    }
    let imports_chirho = &main_chirho.provenance_chirho.imported_references_chirho;
    assert!(!imports_chirho.is_empty());
    for (id_chirho, reference_chirho) in imports_chirho {
        let definition_chirho = helper_chirho
            .bindings_chirho
            .iter()
            .find(|binding_chirho| {
                binding_chirho.binder_chirho.name_chirho == reference_chirho.name_chirho
            })
            .map(|binding_chirho| {
                moved_chirho(binding_chirho.binder_chirho.id_chirho, offsets_chirho[0])
            })
            .unwrap_or_else(|| panic!("the dependency defines {}", reference_chirho.name_chirho));
        assert_eq!(
            record_chirho
                .imported_references_chirho
                .get(&definition_chirho),
            Some(reference_chirho)
        );
        assert!(referenced_chirho.contains(&definition_chirho.0));
        assert!(!referenced_chirho.contains(&moved_chirho(*id_chirho, offsets_chirho[1]).0));
    }

    // A builtin reference moves by its module's offset, or follows the remap to a compiler body
    // of its own name, never to a program binding (#32386).
    for (input_chirho, offset_chirho) in inputs_chirho.iter().zip(&offsets_chirho) {
        for id_chirho in &input_chirho.provenance_chirho.builtin_references_chirho {
            let name_chirho = &input_chirho.names_chirho[id_chirho];
            let moved_id_chirho = moved_chirho(*id_chirho, *offset_chirho);
            assert!(
                record_chirho
                    .builtin_references_chirho
                    .contains(&moved_id_chirho)
                    || merged_chirho.bindings_chirho.iter().any(|binding_chirho| {
                        &binding_chirho.binder_chirho.name_chirho == name_chirho
                            && record_chirho
                                .builtin_references_chirho
                                .contains(&binding_chirho.binder_chirho.id_chirho)
                            && record_chirho
                                .compiler_bindings_chirho
                                .contains(&binding_chirho.binder_chirho.id_chirho)
                    }),
                "the builtin reference {name_chirho} is kept"
            );
        }
    }
}

#[test]
fn a_merged_programs_own_spellings_keep_their_bodies_beside_generated_rows_chirho() {
    // The dependency's own `seq` and `undefined` keep their bodies (42, 7, 17). The generated
    // `force` and `deepseq` keep their primitives: owned, their bodies would fail on `seq#`.
    let (_, machine_chirho) = eval_modules_with_machine_chirho(
        &[
            ("OwnSpellingsChirho.hs", OWN_SPELLINGS_MODULE_CHIRHO),
            ("Main.hs", OWN_SPELLINGS_MAIN_CHIRHO),
        ],
        &mut SourceMapChirho::new_chirho(),
        None,
    )
    .unwrap_or_else(|error_chirho| panic!("the merged program runs: {error_chirho}"));
    assert_eq!(machine_chirho.io_output_chirho, "42\n7\n17\nforced\n");
}

#[test]
fn an_unimported_namesake_never_takes_a_builtin_reference_chirho() {
    let results_chirho = compile_modules_chirho(
        &[
            ("OwnUndefinedChirho.hs", UNIMPORTED_UNDEFINED_MODULE_CHIRHO),
            ("Main.hs", PRELUDE_UNDEFINED_MAIN_CHIRHO),
        ],
        &mut SourceMapChirho::new_chirho(),
    )
    .unwrap_or_else(|refusal_chirho| panic!("GHC accepts the program: {refusal_chirho:?}"));
    let inputs_chirho: Vec<CoreModuleChirho> = results_chirho
        .iter()
        .map(|result_chirho| result_chirho.core_chirho.clone())
        .collect();
    let (merged_chirho, newtype_cons_chirho) =
        merge_interpreter_modules_chirho(results_chirho).expect("the modules merge");
    let offsets_chirho = landed_offsets_chirho(&inputs_chirho, &merged_chirho);
    let [dependency_chirho, main_chirho] = inputs_chirho.as_slice() else {
        panic!("two modules");
    };
    let record_chirho = &merged_chirho.provenance_chirho;

    // The root's `undefined` is a builtin reference and keeps its own id. The dependency's
    // namesake is the program's, and nothing links the root to it.
    let builtin_chirho: Vec<CoreIdChirho> = main_chirho
        .provenance_chirho
        .builtin_references_chirho
        .iter()
        .filter(|id_chirho| {
            main_chirho
                .names_chirho
                .get(id_chirho)
                .is_some_and(|name_chirho| name_chirho == "undefined")
        })
        .map(|id_chirho| moved_chirho(*id_chirho, offsets_chirho[1]))
        .collect();
    assert!(
        !builtin_chirho.is_empty(),
        "the root's undefined is a builtin reference: {:?}",
        main_chirho.provenance_chirho.builtin_references_chirho
    );
    let own_chirho = dependency_chirho
        .bindings_chirho
        .iter()
        .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == "undefined")
        .map(|binding_chirho| {
            moved_chirho(binding_chirho.binder_chirho.id_chirho, offsets_chirho[0])
        })
        .expect("the dependency's own undefined");
    assert!(!record_chirho.compiler_bindings_chirho.contains(&own_chirho));
    assert!(
        !record_chirho
            .builtin_references_chirho
            .contains(&own_chirho)
    );
    let mut root_referenced_chirho = BTreeSet::new();
    for binding_chirho in &merged_chirho.bindings_chirho[dependency_chirho.bindings_chirho.len()..]
    {
        referenced_ids_chirho(&binding_chirho.rhs_chirho, &mut root_referenced_chirho);
    }
    assert!(
        !root_referenced_chirho.contains(&own_chirho.0),
        "the root never reaches the dependency's undefined"
    );
    for id_chirho in &builtin_chirho {
        assert!(record_chirho.builtin_references_chirho.contains(id_chirho));
        assert!(
            root_referenced_chirho.contains(&id_chirho.0),
            "the root keeps its builtin undefined"
        );
    }

    // The run: the dependency's own value, then the Prelude's failure.
    let outcome_chirho = lower_and_run_outcome_chirho(&merged_chirho, None, newtype_cons_chirho)
        .expect("the program lowers");
    assert_eq!(outcome_chirho.machine_chirho.io_output_chirho, "7\n");
    let error_chirho = outcome_chirho
        .result_chirho
        .expect_err("the Prelude's undefined fails");
    assert!(error_chirho.contains("Prelude.undefined"), "{error_chirho}");
}
